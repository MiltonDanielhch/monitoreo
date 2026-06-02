// crates/infrastructure/src/discovery/oui_lookup.rs
// Servicio de lookup OUI para identificar fabricantes por dirección MAC
// Vinculado con ADR-0001 (Dominio Puro)
// Los datos OUI están embebidos como constantes estáticas en el binario

use std::collections::HashMap;

/// Servicio de lookup OUI - datos estáticos embebidos en el binario
/// Formato: (oui_prefix: &str, manufacturer: &str)
/// Los 6 primeros caracteres de una MAC (XX:XX:XX) identifican al fabricante
#[derive(Clone)]
pub struct OuiLookupService {
    oui_db: HashMap<String, &'static str>,
}

impl OuiLookupService {
    /// Crea una nueva instancia con la base de datos OUI embebida
    pub fn new() -> Self {
        let oui_db = Self::build_oui_database();
        Self { oui_db }
    }

    /// Busca el fabricante para una dirección MAC dada
    /// Acepta formato "XX:XX:XX:XX:XX:XX" o "XXXXXXXXXXXX"
    pub fn get_manufacturer(&self, mac: &str) -> Option<&'static str> {
        let oui_prefix = Self::extract_oui_prefix(mac)?;
        self.oui_db.get(&oui_prefix).copied()
    }

    /// Extrae el prefijo OUI (primeros 6 caracteres) de una MAC address
    fn extract_oui_prefix(mac: &str) -> Option<String> {
        let normalized: String = mac
            .to_uppercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .take(6)
            .collect();
        if normalized.len() == 6 {
            Some(normalized)
        } else {
            None
        }
    }

    /// Construye la base de datos OUI estática (subset de IEEE OUI)
    fn build_oui_database() -> HashMap<String, &'static str> {
        let mut db = HashMap::new();

        // Cisco
        db.insert("00001A".into(), "Cisco Systems");
        db.insert("000036".into(), "Cisco Systems");
        db.insert("00004F".into(), "Cisco Systems");
        db.insert("000062".into(), "Cisco Systems");
        db.insert("000067".into(), "Cisco Systems");
        db.insert("00006A".into(), "Cisco Systems");
        db.insert("0000BB".into(), "Cisco Systems");
        db.insert("0011BB".into(), "Cisco Systems");
        db.insert("0014F1".into(), "Cisco Systems");
        db.insert("001A2F".into(), "Cisco Systems");
        db.insert("001B54".into(), "Cisco Systems");
        db.insert("001D46".into(), "Cisco Systems");
        db.insert("001E68".into(), "Cisco Systems");
        db.insert("001E6C".into(), "Cisco Systems");
        db.insert("001F9E".into(), "Cisco Systems");
        db.insert("0022BD".into(), "Cisco Systems");
        db.insert("0023EB".into(), "Cisco Systems");
        db.insert("0024B2".into(), "Cisco Systems");
        db.insert("0025B3".into(), "Cisco Systems");
        db.insert("002645".into(), "Cisco Systems");
        db.insert("002710".into(), "Cisco Systems");
        db.insert("002718".into(), "Cisco Systems");
        db.insert("002764".into(), "Cisco Systems");
        db.insert("C89BC7".into(), "Cisco Systems");

        // HP / HPE
        db.insert("0010E3".into(), "Hewlett Packard");
        db.insert("0011D8".into(), "Hewlett Packard");
        db.insert("001279".into(), "Hewlett Packard");
        db.insert("001E0B".into(), "Hewlett Packard");
        db.insert("001F29".into(), "Hewlett Packard");
        db.insert("0024B2".into(), "Hewlett Packard");
        db.insert("B4B52F".into(), "Hewlett Packard");

        // Dell
        db.insert("0001E9".into(), "Dell");
        db.insert("0002A0".into(), "Dell");
        db.insert("00036F".into(), "Dell");
        db.insert("00188B".into(), "Dell");
        db.insert("001CC8".into(), "Dell");
        db.insert("001D09".into(), "Dell");
        db.insert("001E4F".into(), "Dell");
        db.insert("001EC9".into(), "Dell");
        db.insert("00B0A9".into(), "Dell");
        db.insert("14FE45".into(), "Dell");

        // Lenovo
        db.insert("00074A".into(), "Lenovo");
        db.insert("001E65".into(), "Lenovo");
        db.insert("0003FF".into(), "Lenovo");
        db.insert("00E04C".into(), "Lenovo");
        db.insert("54EE57".into(), "Lenovo");

        // IBM
        db.insert("0005F2".into(), "IBM");
        db.insert("0015F0".into(), "IBM");
        db.insert("0016CB".into(), "IBM");
        db.insert("0017F9".into(), "IBM");

        // VMware
        db.insert("000505".into(), "VMware");
        db.insert("000C29".into(), "VMware");
        db.insert("005056".into(), "VMware");

        // Oracle / Sun
        db.insert("00144F".into(), "Oracle");
        db.insert("0019BB".into(), "Oracle");

        // Intel
        db.insert("0001E6".into(), "Intel");
        db.insert("00029C".into(), "Intel");
        db.insert("0003FF".into(), "Intel");
        db.insert("001F3B".into(), "Intel");
        db.insert("002322".into(), "Intel");
        db.insert("3C97FF".into(), "Intel");

        // Realtek (común en routers económicos y dispositivos IoT)
        db.insert("00061B".into(), "Realtek");
        db.insert("001D0F".into(), "Realtek");
        db.insert("002354".into(), "Realtek");
        db.insert("52D602".into(), "Realtek");
        db.insert("5C4CAD".into(), "Realtek");
        db.insert("6C5F5C".into(), "Realtek");
        db.insert("9CEB4F".into(), "Realtek");
        db.insert("BC0F6B".into(), "Realtek");
        db.insert("C0C5D5".into(), "Realtek");
        db.insert("D850E6".into(), "Realtek");

        // TP-Link
        db.insert("14144B".into(), "TP-Link");
        db.insert("14760B".into(), "TP-Link");
        db.insert("148461".into(), "TP-Link");
        db.insert("14CC20".into(), "TP-Link");
        db.insert("14D6DF".into(), "TP-Link");
        db.insert("18A6F7".into(), "TP-Link");
        db.insert("1C3BF3".into(), "TP-Link");
        db.insert("20F08A".into(), "TP-Link");
        db.insert("30B5C2".into(), "TP-Link");
        db.insert("50C7BF".into(), "TP-Link");
        db.insert("5413E8".into(), "TP-Link");
        db.insert("5C895C".into(), "TP-Link");
        db.insert("60E327".into(), "TP-Link");
        db.insert("78A106".into(), "TP-Link");
        db.insert("90F651".into(), "TP-Link");
        db.insert("A0249F".into(), "TP-Link");
        db.insert("AC841F".into(), "TP-Link");
        db.insert("B0BE76".into(), "TP-Link");
        db.insert("C006C3".into(), "TP-Link");
        db.insert("CC32E5".into(), "TP-Link");
        db.insert("D8068E".into(), "TP-Link");
        db.insert("F48CEB".into(), "TP-Link");
        db.insert("F4EC38".into(), "TP-Link");

        // Huawei
        db.insert("0021F8".into(), "Huawei");
        db.insert("00259C".into(), "Huawei");
        db.insert("20F08A".into(), "Huawei");
        db.insert("2835EE".into(), "Huawei");
        db.insert("38ED18".into(), "Huawei");
        db.insert("3C4755".into(), "Huawei");
        db.insert("4C1FCD".into(), "Huawei");
        db.insert("5C4C2A".into(), "Huawei");
        db.insert("5C7D1E".into(), "Huawei");
        db.insert("6C9D1D".into(), "Huawei");
        db.insert("706B4F".into(), "Huawei");
        db.insert("7489F6".into(), "Huawei");
        db.insert("7C60EB".into(), "Huawei");
        db.insert("804A76".into(), "Huawei");
        db.insert("886B6E".into(), "Huawei");
        db.insert("8C34FD".into(), "Huawei");
        db.insert("9C9E94".into(), "Huawei");
        db.insert("A0F3C1".into(), "Huawei");
        db.insert("AC4E91".into(), "Huawei");
        db.insert("B0A5BD".into(), "Huawei");
        db.insert("C89FF2".into(), "Huawei");
        db.insert("D4C9EF".into(), "Huawei");
        db.insert("E48D5C".into(), "Huawei");
        db.insert("F0C437".into(), "Huawei");

        // MikroTik
        db.insert("00272D".into(), "MikroTik");
        db.insert("082797".into(), "MikroTik");
        db.insert("183A4E".into(), "MikroTik");
        db.insert("242A3B".into(), "MikroTik");
        db.insert("2C21B0".into(), "MikroTik");
        db.insert("2C6BFB".into(), "MikroTik");
        db.insert("305E52".into(), "MikroTik");
        db.insert("4C5E0C".into(), "MikroTik");
        db.insert("50FF69".into(), "MikroTik");
        db.insert("5C5EAB".into(), "MikroTik");
        db.insert("64392A".into(), "MikroTik");
        db.insert("6C9F06".into(), "MikroTik");
        db.insert("78846E".into(), "MikroTik");
        db.insert("84252E".into(), "MikroTik");
        db.insert("84698C".into(), "MikroTik");
        db.insert("885A73".into(), "MikroTik");
        db.insert("8C55B3".into(), "MikroTik");
        db.insert("94659D".into(), "MikroTik");
        db.insert("A020A6".into(), "MikroTik");
        db.insert("AC9F65".into(), "MikroTik");
        db.insert("B8AC6F".into(), "MikroTik");

        // Ubiquiti
        db.insert("18E829".into(), "Ubiquiti");
        db.insert("24A43C".into(), "Ubiquiti");
        db.insert("2CF05D".into(), "Ubiquiti");
        db.insert("44778D".into(), "Ubiquiti");
        db.insert("503DAA".into(), "Ubiquiti");
        db.insert("50D563".into(), "Ubiquiti");
        db.insert("585BDA".into(), "Ubiquiti");
        db.insert("68D79A".into(), "Ubiquiti");
        db.insert("74ACBF".into(), "Ubiquiti");
        db.insert("78303C".into(), "Ubiquiti");
        db.insert("7C6DF4".into(), "Ubiquiti");
        db.insert("98671A".into(), "Ubiquiti");
        db.insert("A03F41".into(), "Ubiquiti");
        db.insert("AC8B13".into(), "Ubiquiti");
        db.insert("DC9FDB".into(), "Ubiquiti");
        db.insert("E0B952".into(), "Ubiquiti");
        db.insert("F0B429".into(), "Ubiquiti");

        // ZTE
        db.insert("38539D".into(), "ZTE");
        db.insert("5475D0".into(), "ZTE");
        db.insert("5C0E6B".into(), "ZTE");
        db.insert("8873EE".into(), "ZTE");
        db.insert("A02C84".into(), "ZTE");
        db.insert("AC23D9".into(), "ZTE");
        db.insert("B0A7D4".into(), "ZTE");
        db.insert("BCE1B9".into(), "ZTE");
        db.insert("C4D2F1".into(), "ZTE");
        db.insert("C8329B".into(), "ZTE");

        // Samsung
        db.insert("0015F9".into(), "Samsung");
        db.insert("002332".into(), "Samsung");
        db.insert("002422".into(), "Samsung");
        db.insert("002708".into(), "Samsung");
        db.insert("003018".into(), "Samsung");
        db.insert("00502F".into(), "Samsung");
        db.insert("009C65".into(), "Samsung");
        db.insert("00E0FC".into(), "Samsung");

        // Apple
        db.insert("000393".into(), "Apple");
        db.insert("000407".into(), "Apple");
        db.insert("0009BB".into(), "Apple");
        db.insert("000D93".into(), "Apple");
        db.insert("001124".into(), "Apple");
        db.insert("001451".into(), "Apple");
        db.insert("0016CB".into(), "Apple");
        db.insert("0017F2".into(), "Apple");
        db.insert("0019E3".into(), "Apple");
        db.insert("001B63".into(), "Apple");
        db.insert("001C23".into(), "Apple");
        db.insert("001D4F".into(), "Apple");
        db.insert("001EC2".into(), "Apple");
        db.insert("001F5B".into(), "Apple");
        db.insert("002241".into(), "Apple");
        db.insert("002436".into(), "Apple");
        db.insert("0024E4".into(), "Apple");
        db.insert("002608".into(), "Apple");
        db.insert("0026B3".into(), "Apple");
        db.insert("0026FD".into(), "Apple");
        db.insert("0027F8".into(), "Apple");
        db.insert("002D21".into(), "Apple");
        db.insert("003065".into(), "Apple");
        db.insert("00339E".into(), "Apple");
        db.insert("0037D4".into(), "Apple");
        db.insert("0050E4".into(), "Apple");
        db.insert("006171".into(), "Apple");
        db.insert("080007".into(), "Apple");

        // Microsoft
        db.insert("00055A".into(), "Microsoft");
        db.insert("000D3A".into(), "Microsoft");
        db.insert("002248".into(), "Microsoft");
        db.insert("00B222".into(), "Microsoft");
        db.insert("282578".into(), "Microsoft");
        db.insert("3CDD0B".into(), "Microsoft");
        db.insert("4829B2".into(), "Microsoft");
        db.insert("504B4E".into(), "Microsoft");
        db.insert("5CBAE7".into(), "Microsoft");
        db.insert("64257E".into(), "Microsoft");
        db.insert("C02C70".into(), "Microsoft");
        db.insert("C09F40".into(), "Microsoft");
        db.insert("B0359F".into(), "Microsoft");
        db.insert("E8B15D".into(), "Microsoft");
        db.insert("F8DB88".into(), "Microsoft");

        // Raspberry Pi
        db.insert("0014AB".into(), "Raspberry Pi");
        db.insert("28CDCC".into(), "Raspberry Pi");
        db.insert("2C3997".into(), "Raspberry Pi");
        db.insert("3C1A86".into(), "Raspberry Pi");
        db.insert("4C11BB".into(), "Raspberry Pi");
        db.insert("54859A".into(), "Raspberry Pi");
        db.insert("7811EB".into(), "Raspberry Pi");
        db.insert("8C16D7".into(), "Raspberry Pi");
        db.insert("A4F1E8".into(), "Raspberry Pi");
        db.insert("B827EB".into(), "Raspberry Pi");
        db.insert("DCA632".into(), "Raspberry Pi");
        db.insert("E45F01".into(), "Raspberry Pi");
        db.insert("E8DE27".into(), "Raspberry Pi");

        // Impresoras
        db.insert("0018F8".into(), "Dell Printer");
        db.insert("002A2A".into(), "Epson");
        db.insert("008810".into(), "Canon");
        db.insert("00E04A".into(), "HP");
        db.insert("AC3F4C".into(), "HP");
        db.insert("C8CB7C".into(), "Canon");

        // Google Android
        db.insert("001566".into(), "Google Android");
        db.insert("0019B9".into(), "Google Android");
        db.insert("001D18".into(), "Google Android");
        db.insert("003481".into(), "Google Android");
        db.insert("0C7418".into(), "Google Android");
        db.insert("1C5F2B".into(), "Google Android");
        db.insert("24DA66".into(), "Google Android");
        db.insert("28ED1A".into(), "Google Android");
        db.insert("34A395".into(), "Google Android");
        db.insert("38084E".into(), "Google Android");
        db.insert("3C6753".into(), "Google Android");
        db.insert("403013".into(), "Google Android");
        db.insert("442A6B".into(), "Google Android");
        db.insert("44765D".into(), "Google Android");
        db.insert("4484F1".into(), "Google Android");
        db.insert("4C48E9".into(), "Google Android");
        db.insert("5008D5".into(), "Google Android");
        db.insert("5084A8".into(), "Google Android");
        db.insert("50A231".into(), "Google Android");
        db.insert("50C7B6".into(), "Google Android");
        db.insert("5189D8".into(), "Google Android");
        db.insert("54084E".into(), "Google Android");
        db.insert("54B4F3".into(), "Google Android");
        db.insert("56318D".into(), "Google Android");
        db.insert("587A67".into(), "Google Android");
        db.insert("58A7C7".into(), "Google Android");
        db.insert("5C2A81".into(), "Google Android");
        db.insert("5C3CA9".into(), "Google Android");
        db.insert("5C5077".into(), "Google Android");
        db.insert("5C8AFF".into(), "Google Android");
        db.insert("5CD233".into(), "Google Android");
        db.insert("5CF5DA".into(), "Google Android");
        db.insert("5E8D94".into(), "Google Android");
        db.insert("60927A".into(), "Google Android");
        db.insert("60D5A4".into(), "Google Android");
        db.insert("6409CF".into(), "Google Android");
        db.insert("647A7B".into(), "Google Android");
        db.insert("649AA5".into(), "Google Android");
        db.insert("64A4C5".into(), "Google Android");
        db.insert("64B3E9".into(), "Google Android");
        db.insert("64CC6B".into(), "Google Android");
        db.insert("680AF8".into(), "Google Android");
        db.insert("6844F6".into(), "Google Android");
        db.insert("6884E5".into(), "Google Android");
        db.insert("68A86D".into(), "Google Android");
        db.insert("6C1872".into(), "Google Android");
        db.insert("6C296B".into(), "Google Android");
        db.insert("6C6237".into(), "Google Android");
        db.insert("6C5D3F".into(), "Google Android");
        db.insert("6C81B3".into(), "Google Android");
        db.insert("6C900D".into(), "Google Android");
        db.insert("6C9B38".into(), "Google Android");
        db.insert("6CA7CF".into(), "Google Android");
        db.insert("7038AC".into(), "Google Android");
        db.insert("705AB2".into(), "Google Android");
        db.insert("70B3D5".into(), "Google Android");
        db.insert("70CC5E".into(), "Google Android");
        db.insert("70CD60".into(), "Google Android");
        db.insert("70DA8B".into(), "Google Android");
        db.insert("7413FC".into(), "Google Android");
        db.insert("745C0E".into(), "Google Android");
        db.insert("7465E3".into(), "Google Android");
        db.insert("74A44A".into(), "Google Android");
        db.insert("74C246".into(), "Google Android");
        db.insert("74E1B6".into(), "Google Android");
        db.insert("7810E4".into(), "Google Android");
        db.insert("7884BC".into(), "Google Android");
        db.insert("78A96E".into(), "Google Android");
        db.insert("78D75F".into(), "Google Android");
        db.insert("78EFC5".into(), "Google Android");
        db.insert("7C2061".into(), "Google Android");
        db.insert("7C5F45".into(), "Google Android");
        db.insert("7C7D47".into(), "Google Android");
        db.insert("7C8BE2".into(), "Google Android");
        db.insert("7C9B98".into(), "Google Android");
        db.insert("7CC2E2".into(), "Google Android");
        db.insert("7CEA80".into(), "Google Android");
        db.insert("7CF0F3".into(), "Google Android");
        db.insert("801ABA".into(), "Google Android");
        db.insert("804B80".into(), "Google Android");
        db.insert("806BEC".into(), "Google Android");
        db.insert("807FC5".into(), "Google Android");
        db.insert("80E562".into(), "Google Android");
        db.insert("843835".into(), "Google Android");
        db.insert("844A3B".into(), "Google Android");
        db.insert("8460E7".into(), "Google Android");
        db.insert("8465F9".into(), "Google Android");
        db.insert("846996".into(), "Google Android");
        db.insert("8481F3".into(), "Google Android");
        db.insert("849C57".into(), "Google Android");
        db.insert("84A6C6".into(), "Google Android");
        db.insert("84C18E".into(), "Google Android");
        db.insert("84E5A6".into(), "Google Android");
        db.insert("8425DB".into(), "Google Android");
        db.insert("848E29".into(), "Google Android");
        db.insert("880632".into(), "Google Android");
        db.insert("88308B".into(), "Google Android");
        db.insert("885828".into(), "Google Android");
        db.insert("885A08".into(), "Google Android");
        db.insert("8863A1".into(), "Google Android");
        db.insert("8866C7".into(), "Google Android");
        db.insert("887591".into(), "Google Android");
        db.insert("88769A".into(), "Google Android");
        db.insert("888322".into(), "Google Android");
        db.insert("88B1CC".into(), "Google Android");
        db.insert("88B3AB".into(), "Google Android");
        db.insert("88C263".into(), "Google Android");
        db.insert("88CB05".into(), "Google Android");
        db.insert("88D858".into(), "Google Android");
        db.insert("88E6BA".into(), "Google Android");
        db.insert("8C1D68".into(), "Google Android");
        db.insert("8C3A35".into(), "Google Android");
        db.insert("8C5C8E".into(), "Google Android");
        db.insert("8C7B9D".into(), "Google Android");
        db.insert("8C7C92".into(), "Google Android");
        db.insert("8C88CC".into(), "Google Android");
        db.insert("8C9166".into(), "Google Android");
        db.insert("8C9EFC".into(), "Google Android");
        db.insert("8CA5A8".into(), "Google Android");
        db.insert("8CE0DA".into(), "Google Android");
        db.insert("9027E4".into(), "Google Android");
        db.insert("902A96".into(), "Google Android");
        db.insert("90408D".into(), "Google Android");
        db.insert("90549C".into(), "Google Android");
        db.insert("90659D".into(), "Google Android");
        db.insert("90696A".into(), "Google Android");
        db.insert("906C05".into(), "Google Android");
        db.insert("90B00C".into(), "Google Android");
        db.insert("90B9E7".into(), "Google Android");
        db.insert("90CC70".into(), "Google Android");
        db.insert("90F652".into(), "Google Android");
        db.insert("940C7C".into(), "Google Android");
        db.insert("944852".into(), "Google Android");
        db.insert("947F78".into(), "Google Android");
        db.insert("9482B7".into(), "Google Android");
        db.insert("949144".into(), "Google Android");
        db.insert("94B1D0".into(), "Google Android");
        db.insert("985AED".into(), "Google Android");
        db.insert("9873CC".into(), "Google Android");
        db.insert("987A3E".into(), "Google Android");
        db.insert("989F70".into(), "Google Android");
        db.insert("98C0EB".into(), "Google Android");
        db.insert("98CBFC".into(), "Google Android");
        db.insert("9C1B2A".into(), "Google Android");
        db.insert("9C3B5F".into(), "Google Android");
        db.insert("9C4F87".into(), "Google Android");
        db.insert("9C7B20".into(), "Google Android");
        db.insert("9C7F81".into(), "Google Android");
        db.insert("9C81C2".into(), "Google Android");
        db.insert("9C9E0F".into(), "Google Android");
        db.insert("9CC0D7".into(), "Google Android");
        db.insert("9CD0E9".into(), "Google Android");
        db.insert("9CDBD4".into(), "Google Android");
        db.insert("9CE882".into(), "Google Android");
        db.insert("9CEDFD".into(), "Google Android");
        db.insert("A04E8A".into(), "Google Android");
        db.insert("A0C0B8".into(), "Google Android");
        db.insert("A0D795".into(), "Google Android");
        db.insert("A0E5C7".into(), "Google Android");
        db.insert("A0F3C1".into(), "Google Android");
        db.insert("A45C12".into(), "Google Android");
        db.insert("A47DE0".into(), "Google Android");
        db.insert("A4AABE".into(), "Google Android");
        db.insert("A4B197".into(), "Google Android");
        db.insert("A4B805".into(), "Google Android");
        db.insert("A4B8C5".into(), "Google Android");
        db.insert("A4BB5D".into(), "Google Android");
        db.insert("A4C02E".into(), "Google Android");
        db.insert("A4C3CC".into(), "Google Android");
        db.insert("A4C4A9".into(), "Google Android");
        db.insert("A4C4E8".into(), "Google Android");
        db.insert("A4C515".into(), "Google Android");
        db.insert("A4C539".into(), "Google Android");
        db.insert("A4C605".into(), "Google Android");
        db.insert("A4C6CC".into(), "Google Android");
        db.insert("A4D1D2".into(), "Google Android");
        db.insert("A4D181".into(), "Google Android");
        db.insert("A4D1E8".into(), "Google Android");
        db.insert("A4D237".into(), "Google Android");
        db.insert("A4D2B8".into(), "Google Android");
        db.insert("A4D30D".into(), "Google Android");
        db.insert("A4D405".into(), "Google Android");
        db.insert("A4D4C2".into(), "Google Android");
        db.insert("A4D5CF".into(), "Google Android");
        db.insert("A4D654".into(), "Google Android");
        db.insert("A4D6E7".into(), "Google Android");
        db.insert("A4D71E".into(), "Google Android");
        db.insert("A4D7B9".into(), "Google Android");
        db.insert("A4D847".into(), "Google Android");
        db.insert("A4D878".into(), "Google Android");
        db.insert("A4D904".into(), "Google Android");
        db.insert("A4D91F".into(), "Google Android");
        db.insert("A4DAD7".into(), "Google Android");
        db.insert("A4DB47".into(), "Google Android");
        db.insert("A4DC8D".into(), "Google Android");
        db.insert("A4DD2B".into(), "Google Android");
        db.insert("A4DE47".into(), "Google Android");
        db.insert("A4DEAD".into(), "Google Android");
        db.insert("A4E091".into(), "Google Android");
        db.insert("A4E26F".into(), "Google Android");
        db.insert("A4E34A".into(), "Google Android");
        db.insert("A4E4FC".into(), "Google Android");
        db.insert("A4E7B0".into(), "Google Android");
        db.insert("A4E8C2".into(), "Google Android");
        db.insert("A4EA53".into(), "Google Android");
        db.insert("A4EB93".into(), "Google Android");
        db.insert("A4ED35".into(), "Google Android");
        db.insert("A4EE1A".into(), "Google Android");
        db.insert("A4F063".into(), "Google Android");
        db.insert("A4F4C6".into(), "Google Android");
        db.insert("A4FCAB".into(), "Google Android");
        db.insert("A81B6A".into(), "Google Android");
        db.insert("A832E7".into(), "Google Android");
        db.insert("A8369F".into(), "Google Android");
        db.insert("A8578A".into(), "Google Android");
        db.insert("A85A3F".into(), "Google Android");
        db.insert("A85C07".into(), "Google Android");
        db.insert("A86562".into(), "Google Android");
        db.insert("A86A54".into(), "Google Android");
        db.insert("A86B61".into(), "Google Android");
        db.insert("A86C0A".into(), "Google Android");
        db.insert("A86E7F".into(), "Google Android");
        db.insert("A8710B".into(), "Google Android");
        db.insert("A87780".into(), "Google Android");
        db.insert("A87B3A".into(), "Google Android");
        db.insert("A87E99".into(), "Google Android");
        db.insert("A8815B".into(), "Google Android");
        db.insert("A88208".into(), "Google Android");
        db.insert("A8836E".into(), "Google Android");
        db.insert("A88479".into(), "Google Android");

        db
    }
}

impl Default for OuiLookupService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oui_lookup_cisco() {
        let service = OuiLookupService::new();
        // Formato XX:XX:XX:XX:XX:XX
        assert_eq!(service.get_manufacturer("00:00:1A:12:34:56"), Some("Cisco Systems"));
        // Formato sin separadores
        assert_eq!(service.get_manufacturer("00001A123456"), Some("Cisco Systems"));
    }

    #[test]
    fn test_oui_lookup_unknown() {
        let service = OuiLookupService::new();
        assert_eq!(service.get_manufacturer("FF:FF:FF:12:34:56"), None);
    }

    #[test]
    fn test_oui_lookup_apple() {
        let service = OuiLookupService::new();
        assert_eq!(service.get_manufacturer("00:03:93:12:34:56"), Some("Apple"));
        assert_eq!(service.get_manufacturer("00:09:BB:AA:BB:CC"), Some("Apple"));
    }
}
