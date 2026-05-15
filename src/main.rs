use std::io;

// ============================================================
//  Struct Ruangan — hanya 3 field input utama
// ============================================================
struct Ruangan {
    set_point_lembab: f32,
    temp_aktual: f32,
    humid_aktual: f32,
}

// ============================================================
//  Implementasi Method
// ============================================================
impl Ruangan {
    // Inisialisasi dengan 3 input
    fn new(set_point_lembab: f32, temp_aktual: f32, humid_aktual: f32) -> Ruangan {
        Ruangan {
            set_point_lembab,
            temp_aktual,
            humid_aktual,
        }
    }

    // DHT11: pembacaan sensor = aktual + offset kecil (1 atau 2)
    fn dht11_temp(&self) -> f32 {
        self.temp_aktual + 1.0
    }

    fn dht11_humid(&self) -> f32 {
        self.humid_aktual + 2.0
    }

    // MQ135: CO2 dan NH3 dihitung dari pembacaan humidity DHT11
    fn mq135_co2(&self) -> f32 {
        self.dht11_humid() * 5.0
    }

    fn mq135_nh3(&self) -> f32 {
        self.dht11_humid() * 10.0
    }

    // Cek apakah aktuator alarm harus aktif
    fn aktuator_aktif(&self) -> bool {
        self.humid_aktual > self.set_point_lembab || self.dht11_humid() > self.set_point_lembab
    }

    // Error = pembacaan sensor - aktual
    fn error_temp(&self) -> f32 {
        self.dht11_temp() - self.temp_aktual
    }

    fn error_humid(&self) -> f32 {
        self.dht11_humid() - self.humid_aktual
    }

    // Tampilkan seluruh output
    fn tampilkan(&self) {
        let status_alarm = if self.aktuator_aktif() { "Aktif" } else { "Non-Aktif" };

        println!("\n========================================");
        println!("       DASHBOARD MONITORING RUANGAN     ");
        println!("========================================");

        // --- DHT11 ---
        println!("\n[ Pembacaan Sensor DHT11 ]");
        println!("  Temperature : {:.1} °C", self.dht11_temp());
        println!("  Humidity    : {:.1} %", self.dht11_humid());

        // --- MQ135 ---
        println!("\n[ Pembacaan Sensor MQ135 ]");
        println!("  CO2 : {:.1} ppm", self.mq135_co2());
        println!("  NH3 : {:.1} ppm", self.mq135_nh3());

        // --- Controller ESP32 ---
        println!("\n[ Controller ESP32 ]");
        println!("  Sensor (DHT11 dan MQ135) : Aktif");
        println!("  Aktuator (Alarm)         : {}", status_alarm);

        // --- Alarm ---
        println!("\n[ Alarm ]");
        if self.aktuator_aktif() {
            println!("  Status : AKTIF  !! Humidity melebihi Set Point !!");
        } else {
            println!("  Status : Non-Aktif");
        }

        // --- Error ---
        println!("\n[ Error Sensor ]");
        println!("  Error Temperature : {:.1} °C", self.error_temp());
        println!("  Error Humidity    : {:.1} %", self.error_humid());

        println!("\n========================================\n");
    }
}

// ============================================================
//  Helper input
// ============================================================
fn input_angka(pesan: &str) -> f32 {
    loop {
        println!("{}", pesan);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(angka) = input.trim().parse() {
            return angka;
        } else {
            println!("Input salah! Masukkan angka.");
        }
    }
}

// ============================================================
//  Main
// ============================================================
fn main() {
    println!("========================================");
    println!("    SISTEM MONITORING SENSOR v3.0       ");
    println!("========================================");

    let sp_lembab    = input_angka("\nMasukkan Set Point Humidity (%): ");
    let temp_aktual  = input_angka("Masukkan Temperature Aktual (°C): ");
    let humid_aktual = input_angka("Masukkan Humidity Aktual (%): ");

    let mut ruangan = Ruangan::new(sp_lembab, temp_aktual, humid_aktual);
    ruangan.tampilkan();

    // Loop menu update
    loop {
        println!("MENU: [1] Update Nilai  [2] Keluar");
        let pilihan = input_angka("Pilih (1/2): ");

        if pilihan == 1.0 {
            let t = input_angka("\nTemperature Aktual baru (°C): ");
            let h = input_angka("Humidity Aktual baru (%): ");
            ruangan.temp_aktual  = t;
            ruangan.humid_aktual = h;
            ruangan.tampilkan();
        } else if pilihan == 2.0 {
            println!("Sistem Dimatikan.");
            break;
        } else {
            println!("Pilihan tidak valid.");
        }
    }
}