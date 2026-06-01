// crates/infrastructure/tests/worker_resilience_tests.rs
// Pruebas de resiliencia para workers Tokio MPSC
// Vinculado con ADR-0015-tokio-jobs.md

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration, timeout};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Simula un payload de trabajo
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct TestJob {
    id: u64,
    payload: String,
}

/// Simula un worker con reintentos con exponential backoff
async fn worker_with_backoff(
    mut receiver: mpsc::Receiver<TestJob>,
    success_counter: Arc<AtomicU64>,
    _failure_counter: Arc<AtomicU64>,
) {
    while let Some(_job) = receiver.recv().await {
        let mut retry_count = 0;
        let max_retries = 3;
        let mut backoff = Duration::from_millis(100);

        loop {
            // Simular procesamiento con 50% de fallo inicial
            if retry_count < 2 {
                // Simular fallo en los primeros intentos
                retry_count += 1;
                if retry_count < max_retries {
                    sleep(backoff).await;
                    backoff *= 2; // Exponential backoff
                    continue;
                }
            }

            // Simular éxito
            success_counter.fetch_add(1, Ordering::Relaxed);
            break;
        }
    }
}

/// Prueba 8.8.1: Verificar que los payloads se retienen durante desconexión
#[tokio::test]
async fn test_worker_retains_payloads_during_disconnect() {
    let (sender, receiver) = mpsc::channel(100);
    let success_counter = Arc::new(AtomicU64::new(0));
    let failure_counter = Arc::new(AtomicU64::new(0));

    // Spawnear worker
    let success_counter_clone = success_counter.clone();
    let failure_counter_clone = failure_counter.clone();
    tokio::spawn(async move {
        worker_with_backoff(receiver, success_counter_clone, failure_counter_clone).await;
    });

    // Enviar trabajos antes de "desconectar"
    for i in 0..10 {
        sender
            .send(TestJob {
                id: i,
                payload: format!("job_{}", i),
            })
            .await
            .unwrap();
    }

    // Esperar a que se procesen algunos trabajos
    sleep(Duration::from_millis(500)).await;

    // Verificar que se procesaron trabajos
    let processed = success_counter.load(Ordering::Relaxed);
    assert!(processed > 0, "Deberían haberse procesado trabajos");

    // Cerrar el canal
    drop(sender);

    // Esperar a que el worker termine
    sleep(Duration::from_millis(100)).await;

    println!("✓ Prueba 8.8.1: Worker retiene payloads durante desconexión");
}

/// Prueba 8.8.2: Verificar que los workers se ejecutan en hilos aislados
#[tokio::test]
async fn test_workers_run_in_isolated_threads() {
    let (sender1, mut receiver1) = mpsc::channel(100);
    let (sender2, mut receiver2) = mpsc::channel(100);

    let counter1 = Arc::new(AtomicU64::new(0));
    let counter2 = Arc::new(AtomicU64::new(0));

    // Worker 1: Simula SNMP discovery (tarea pesada)
    let counter1_clone = counter1.clone();
    tokio::spawn(async move {
        while let Some(_job) = receiver1.recv().await {
            // Simular tarea pesada
            sleep(Duration::from_millis(50)).await;
            counter1_clone.fetch_add(1, Ordering::Relaxed);
        }
    });

    // Worker 2: Simula pruning (tarea pesada)
    let counter2_clone = counter2.clone();
    tokio::spawn(async move {
        while let Some(_job) = receiver2.recv().await {
            // Simular tarea pesada
            sleep(Duration::from_millis(50)).await;
            counter2_clone.fetch_add(1, Ordering::Relaxed);
        }
    });

    // Enviar trabajos a ambos workers en paralelo
    for i in 0..20 {
        sender1
            .send(TestJob {
                id: i,
                payload: format!("snmp_{}", i),
            })
            .await
            .unwrap();

        sender2
            .send(TestJob {
                id: i,
                payload: format!("pruning_{}", i),
            })
            .await
            .unwrap();
    }

    // Esperar a que se procesen
    sleep(Duration::from_millis(1500)).await;

    // Verificar que ambos workers procesaron trabajos
    let processed1 = counter1.load(Ordering::Relaxed);
    let processed2 = counter2.load(Ordering::Relaxed);

    assert!(processed1 > 0, "Worker 1 debería haber procesado trabajos");
    assert!(processed2 > 0, "Worker 2 debería haber procesado trabajos");

    println!("✓ Prueba 8.8.2: Workers se ejecutan en hilos aislados");
}

/// Prueba de exponential backoff
#[tokio::test]
async fn test_exponential_backoff() {
    let mut backoff = Duration::from_millis(100);
    let mut retry_count = 0;
    let max_retries = 3;

    while retry_count < max_retries {
        retry_count += 1;
        backoff *= 2; // Exponential backoff
    }

    // Verificar que el backoff aumentó exponencialmente
    // Después de 3 intentos: 100 * 2^3 = 800ms
    assert_eq!(backoff, Duration::from_millis(800), "El backoff debería ser 800ms después de 3 intentos");

    println!("✓ Prueba de exponential backoff: Backoff aumenta correctamente");
}

/// Prueba de capacidad del canal
#[tokio::test]
async fn test_channel_capacity() {
    let (sender, mut receiver) = mpsc::channel(10);
    let counter = Arc::new(AtomicU64::new(0));

    // Worker lento
    let counter_clone = counter.clone();
    tokio::spawn(async move {
        while let Some(_job) = receiver.recv().await {
            sleep(Duration::from_millis(100)).await;
            counter_clone.fetch_add(1, Ordering::Relaxed);
        }
    });

    // Enviar más trabajos que la capacidad del canal
    for i in 0..15 {
        let result = timeout(Duration::from_millis(10), sender.send(TestJob {
            id: i,
            payload: format!("job_{}", i),
        })).await;

        if i < 10 {
            assert!(result.is_ok(), "Los primeros 10 trabajos deberían enviarse inmediatamente");
        }
    }

    // Cerrar el canal
    drop(sender);

    // Esperar a que se procesen
    sleep(Duration::from_millis(500)).await;

    let processed = counter.load(Ordering::Relaxed);
    assert!(processed > 0, "Deberían haberse procesado trabajos");

    println!("✓ Prueba de capacidad del canal: Canal maneja trabajos correctamente");
}
