<!-- apps/web/src/routes/dashboard/infrastructure/+page.svelte -->
<!-- Gestión de archivos de infraestructura - Svelte 5 + TanStack Query + shadcn-svelte -->
<!-- Módulo 5: Infraestructura y Topologías -->

<script lang="ts">
    import { createQuery, createMutation } from '@tanstack/svelte-query';
    import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
    import { Button } from '$lib/components/ui/button';
    import { Badge } from '$lib/components/ui/badge';
    import { Upload, FileImage, FileText, Download, Trash2 } from 'lucide-svelte';

    // Estado de subida
    let uploading = $state(false);
    let progress = $state(0);
    let selectedFile = $state<File | null>(null);
    let fileCategory = $state('TOPOLOGY_SVG');
    let selectedSede = $state('trinidad');
    let viewMode = $state('list'); // 'list' o 'viewer'

    interface FileMetadata {
        id: string;
        filename: string;
        file_type: string;
        file_size_bytes: number;
    }

    interface FileListResponse {
        files: FileMetadata[];
    }

    // Query para obtener archivos
    const filesQuery = createQuery(() => ({
        queryKey: ['infrastructure-files', selectedSede],
        queryFn: async () => {
            const response = await fetch(`/api/v1/infrastructure/files?sede_id=${selectedSede}`);
            if (!response.ok) throw new Error('Error fetching files');
            const data: FileListResponse = await response.json();
            return data.files;
        }
    }));

    // Mutation para subir archivo
    const uploadMutation = createMutation(() => ({
        mutationFn: async (file: File) => {
            const formData = new FormData();
            formData.append('file', file);

            const response = await fetch('/api/v1/infrastructure/upload', {
                method: 'POST',
                body: formData
            });

            if (!response.ok) {
                const error = await response.json();
                throw new Error(error.error || 'Error uploading file');
            }

            return response.json();
        },
        onMutate: () => {
            uploading = true;
            progress = 0;
        },
        onSuccess: () => {
            uploading = false;
            progress = 100;
            selectedFile = null;
            filesQuery.refetch();
        },
        onError: (error: Error) => {
            uploading = false;
            progress = 0;
            console.error('Upload error:', error);
        }
    }));

    // Función para manejar selección de archivo
    function handleFileSelect(event: Event) {
        const target = event.target as HTMLInputElement;
        if (target.files && target.files[0]) {
            selectedFile = target.files[0];
        }
    }

    // Función para validar extensión
    function validateFile(file: File): boolean {
        const lower = file.name.toLowerCase();
        if (fileCategory === 'TOPOLOGY_SVG') {
            return lower.endsWith('.svg');
        } else if (fileCategory === 'RACK_IMAGE') {
            return lower.endsWith('.png') || lower.endsWith('.jpg') || lower.endsWith('.jpeg');
        } else if (fileCategory === 'CONFIG_BACKUP') {
            return lower.endsWith('.cfg') || lower.endsWith('.txt');
        }
        return false;
    }

    // Función para subir archivo
    async function handleUpload() {
        if (!selectedFile) return;

        if (!validateFile(selectedFile)) {
            alert('Tipo de archivo no válido para la categoría seleccionada');
            return;
        }

        uploadMutation.mutate(selectedFile);
    }

    // Función para descargar archivo
    async function handleDownload(fileId: string, filename: string) {
        const response = await fetch(`/api/v1/infrastructure/download/${fileId}`);
        if (!response.ok) {
            alert('Error downloading file');
            return;
        }

        const blob = await response.blob();
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = filename;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        window.URL.revokeObjectURL(url);
    }

    // Función para formatear tamaño
    function formatFileSize(bytes: number): string {
        if (bytes === 0) return '0 Bytes';
        const k = 1024;
        const sizes = ['Bytes', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
    }

    // Función para obtener icono según tipo
    function getFileIcon(fileType: string) {
        if (fileType === 'TOPOLOGY_SVG') return FileText;
        if (fileType === 'RACK_IMAGE') return FileImage;
        return FileText;
    }

    // Función para renderizar icono
    function renderIcon(fileType: string) {
        if (fileType === 'TOPOLOGY_SVG') return FileText;
        if (fileType === 'RACK_IMAGE') return FileImage;
        return FileText;
    }

    // Función para obtener color según tipo
    function getFileTypeColor(fileType: string): string {
        if (fileType === 'TOPOLOGY_SVG') return 'bg-blue-500';
        if (fileType === 'RACK_IMAGE') return 'bg-green-500';
        if (fileType === 'CONFIG_BACKUP') return 'bg-orange-500';
        return 'bg-gray-500';
    }

    // Función para obtener archivos por tipo
    function getFilesByType(fileType: string): FileMetadata[] {
        if (!filesQuery.data) return [];
        return filesQuery.data.filter(f => f.file_type === fileType);
    }

    // Función para ver topología SVG
    async function viewTopology(fileId: string) {
        const response = await fetch(`/api/v1/infrastructure/download/${fileId}`);
        if (!response.ok) {
            alert('Error loading topology');
            return;
        }
        const svgContent = await response.text();
        // Mostrar en un modal o visor
        alert('SVG viewer would open here with content length: ' + svgContent.length);
    }
</script>

<div class="p-6 space-y-6">
    <div class="flex items-center justify-between">
        <div>
            <h1 class="text-3xl font-bold text-zinc-100">Infraestructura</h1>
            <p class="text-zinc-400 mt-1">Gestión de archivos técnicos y topologías de red</p>
        </div>
        
        <!-- Selector de sedes -->
        <div class="flex gap-2">
            <Button
                variant={selectedSede === 'trinidad' ? 'default' : 'outline'}
                onclick={() => selectedSede = 'trinidad'}
                class={selectedSede === 'trinidad' ? 'bg-blue-600' : 'border-zinc-700 text-zinc-300'}
            >
                Trinidad
            </Button>
            <Button
                variant={selectedSede === 'riberalta' ? 'default' : 'outline'}
                onclick={() => selectedSede = 'riberalta'}
                class={selectedSede === 'riberalta' ? 'bg-blue-600' : 'border-zinc-700 text-zinc-300'}
            >
                Riberalta
            </Button>
            <Button
                variant={selectedSede === 'guayaramerin' ? 'default' : 'outline'}
                onclick={() => selectedSede = 'guayaramerin'}
                class={selectedSede === 'guayaramerin' ? 'bg-blue-600' : 'border-zinc-700 text-zinc-300'}
            >
                Guayaramerín
            </Button>
        </div>
    </div>

    <!-- Visor de topología SVG -->
    {#if getFilesByType('TOPOLOGY_SVG').length > 0}
        <Card class="bg-zinc-900 border-zinc-800">
            <CardHeader>
                <CardTitle class="text-zinc-100">Topología de Red</CardTitle>
                <CardDescription class="text-zinc-400">
                    Diagrama SVG de la topología de red de {selectedSede}
                </CardDescription>
            </CardHeader>
            <CardContent>
                <div class="bg-zinc-800 rounded-lg p-4 flex items-center justify-center min-h-[400px]">
                    {#if getFilesByType('TOPOLOGY_SVG').length === 1}
                        <Button
                            onclick={() => viewTopology(getFilesByType('TOPOLOGY_SVG')[0].id)}
                            class="bg-blue-600 hover:bg-blue-700"
                        >
                            <FileText class="h-4 w-4 mr-2" />
                            Ver Topología
                        </Button>
                    {:else}
                        <div class="grid grid-cols-2 gap-4 w-full">
                            {#each getFilesByType('TOPOLOGY_SVG') as file}
                                <Button
                                    onclick={() => viewTopology(file.id)}
                                    variant="outline"
                                    class="border-zinc-700 text-zinc-300"
                                >
                                    <FileText class="h-4 w-4 mr-2" />
                                    {file.filename}
                                </Button>
                            {/each}
                        </div>
                    {/if}
                </div>
            </CardContent>
        </Card>
    {/if}

    <!-- Galería de racks -->
    {#if getFilesByType('RACK_IMAGE').length > 0}
        <Card class="bg-zinc-900 border-zinc-800">
            <CardHeader>
                <CardTitle class="text-zinc-100">Galería de Racks</CardTitle>
                <CardDescription class="text-zinc-400">
                    Imágenes de los racks de servidores de {selectedSede}
                </CardDescription>
            </CardHeader>
            <CardContent>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                    {#each getFilesByType('RACK_IMAGE') as file}
                        <div class="bg-zinc-800 rounded-lg p-2 hover:bg-zinc-750 transition-colors cursor-pointer">
                            <div class="aspect-square bg-zinc-700 rounded mb-2 flex items-center justify-center">
                                <FileImage class="h-12 w-12 text-zinc-500" />
                            </div>
                            <p class="text-zinc-300 text-sm text-center truncate">{file.filename}</p>
                        </div>
                    {/each}
                </div>
            </CardContent>
        </Card>
    {/if}

    <!-- Zona de carga -->
    <Card class="bg-zinc-900 border-zinc-800">
        <CardHeader>
            <CardTitle class="text-zinc-100">Subir Archivo</CardTitle>
            <CardDescription class="text-zinc-400">
                Arrastra archivos o selecciona para subir planos SVG, imágenes de racks o respaldos de configuración
            </CardDescription>
        </CardHeader>
        <CardContent class="space-y-4">
            <!-- Selector de categoría -->
            <div class="flex gap-2">
                <Button
                    variant={fileCategory === 'TOPOLOGY_SVG' ? 'default' : 'outline'}
                    onclick={() => fileCategory = 'TOPOLOGY_SVG'}
                    class={fileCategory === 'TOPOLOGY_SVG' ? 'bg-blue-600' : 'border-zinc-700 text-zinc-300'}
                >
                    Topología SVG
                </Button>
                <Button
                    variant={fileCategory === 'RACK_IMAGE' ? 'default' : 'outline'}
                    onclick={() => fileCategory = 'RACK_IMAGE'}
                    class={fileCategory === 'RACK_IMAGE' ? 'bg-green-600' : 'border-zinc-700 text-zinc-300'}
                >
                    Imagen de Rack
                </Button>
                <Button
                    variant={fileCategory === 'CONFIG_BACKUP' ? 'default' : 'outline'}
                    onclick={() => fileCategory = 'CONFIG_BACKUP'}
                    class={fileCategory === 'CONFIG_BACKUP' ? 'bg-orange-600' : 'border-zinc-700 text-zinc-300'}
                >
                    Respaldo CFG
                </Button>
            </div>

            <!-- Zona de arrastre -->
            <div
                class="border-2 border-dashed border-zinc-700 rounded-lg p-8 text-center hover:border-zinc-600 transition-colors"
                class:animate-pulse={uploading}
            >
                <input
                    type="file"
                    id="file-input"
                    accept={fileCategory === 'TOPOLOGY_SVG' ? '.svg' : fileCategory === 'RACK_IMAGE' ? '.png,.jpg,.jpeg' : '.cfg,.txt'}
                    onchange={handleFileSelect}
                    class="hidden"
                />
                <label for="file-input" class="cursor-pointer">
                    <Upload class="h-12 w-12 mx-auto text-zinc-500 mb-4" />
                    <p class="text-zinc-300 mb-2">
                        {selectedFile ? selectedFile.name : 'Arrastra un archivo aquí o haz clic para seleccionar'}
                    </p>
                    <p class="text-zinc-500 text-sm">
                        {fileCategory === 'TOPOLOGY_SVG' ? 'SVG' : fileCategory === 'RACK_IMAGE' ? 'PNG, JPG, JPEG' : 'CFG, TXT'}
                    </p>
                </label>
            </div>

            <!-- Barra de progreso -->
            {#if uploading}
                <div class="w-full bg-zinc-800 rounded-full h-2">
                    <div
                        class="bg-blue-600 h-2 rounded-full transition-all duration-300"
                        style="width: {progress}%"
                    ></div>
                </div>
                <p class="text-zinc-400 text-sm text-center">Subiendo... {progress}%</p>
            {/if}

            <!-- Botón de subir -->
            {#if selectedFile && !uploading}
                <Button
                    onclick={handleUpload}
                    class="w-full bg-blue-600 hover:bg-blue-700"
                    disabled={uploading}
                >
                    Subir Archivo
                </Button>
            {/if}
        </CardContent>
    </Card>

    <!-- Lista de archivos -->
    <Card class="bg-zinc-900 border-zinc-800">
        <CardHeader>
            <CardTitle class="text-zinc-100">Archivos</CardTitle>
            <CardDescription class="text-zinc-400">
                Archivos técnicos de la sede de Trinidad
            </CardDescription>
        </CardHeader>
        <CardContent>
            {#if filesQuery.isLoading}
                <p class="text-zinc-400 text-center py-8">Cargando archivos...</p>
            {:else if filesQuery.isError}
                <p class="text-red-400 text-center py-8">Error al cargar archivos</p>
            {:else if filesQuery.data && filesQuery.data.length > 0}
                <div class="space-y-2">
                    {#each filesQuery.data as file (file.id)}
                        <div class="flex items-center justify-between p-4 bg-zinc-800 rounded-lg hover:bg-zinc-750 transition-colors">
                            <div class="flex items-center gap-3">
                                {#if file.file_type === 'TOPOLOGY_SVG'}
                                    <FileText class="h-5 w-5 text-zinc-400" />
                                {:else if file.file_type === 'RACK_IMAGE'}
                                    <FileImage class="h-5 w-5 text-zinc-400" />
                                {:else}
                                    <FileText class="h-5 w-5 text-zinc-400" />
                                {/if}
                                <div>
                                    <p class="text-zinc-100 font-medium">{file.filename}</p>
                                    <p class="text-zinc-500 text-sm">{formatFileSize(file.file_size_bytes)}</p>
                                </div>
                                <Badge class={getFileTypeColor(file.file_type)}>
                                    {file.file_type}
                                </Badge>
                            </div>
                            <div class="flex gap-2">
                                <Button
                                    variant="ghost"
                                    size="sm"
                                    onclick={() => handleDownload(file.id, file.filename)}
                                    class="text-zinc-400 hover:text-zinc-100"
                                >
                                    <Download class="h-4 w-4" />
                                </Button>
                                <Button
                                    variant="ghost"
                                    size="sm"
                                    class="text-zinc-400 hover:text-red-400"
                                >
                                    <Trash2 class="h-4 w-4" />
                                </Button>
                            </div>
                        </div>
                    {/each}
                </div>
            {:else}
                <p class="text-zinc-500 text-center py-8">No hay archivos cargados</p>
            {/if}
        </CardContent>
    </Card>
</div>
