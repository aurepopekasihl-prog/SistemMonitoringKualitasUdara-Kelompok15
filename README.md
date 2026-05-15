Sistem Monitoring Kualitas Udara Menggunakan 
Sensor 1   : DHT11
Sensor 2   : MQ135
Controller : ESP32
Aktuator   : Alarm

Deskripsi Project
Project ini berbasis dari monitoring kualitas udara didalam ruangan serta monitoring Humidity dan Temperaturenya.
dengan input set point dari humidity,aktual humidity,dan aktual temperature,maka sensor akan melakukan pembacaan dengan output
"Pembacaan Sensor Temperature,Pembacaan Sensor Humidity,CO2,NH3,Controller Aktif/Non-Aktif,Alarm Aktif/Non-Aktif,
dan Kalibrasi Error antara Temperature/Humidity Aktual dan Sensor"

Cara Kerja Programnya
Pertama         : Masukkan Input Berupa Set Point Humidity,Aktual Humidity,Aktual Temperature.

Kedua           : Output akan berupa "Pembacaan Sensor Temperature,Pembacaan Sensor Humidity,CO2,NH3,Controller Aktif/Non-Aktif,Alarm Aktif/Non-Aktif,
dan Kalibrasi Error antara Temperature/Humidity Aktual dan Sensor".

Ketiga          : Apabila Aktual Humidity & Sensor Humidity > Set Point Humidity,Maka controller akan aktif dan juga mengaktifkan Aktuator yaitu Alarm.

Keempat         : Lalu akan ada menu pengulangan dan keluar,jika mau mengulangi ketik 1 dan isi input dari Set Point Humidity,Aktual Humidity, dan Aktual Temperature.

Berikut Adalah Screenshot Hasil Program Yang Telah Kami Buat

<img width="555" height="525" alt="WhatsApp Image 2026-05-15 at 12 47 32" src="https://github.com/user-attachments/assets/760f880c-8094-4d81-8444-eeb2277969de" />
<img width="515" height="593" alt="WhatsApp Image 2026-05-15 at 12 47 32 (1)" src="https://github.com/user-attachments/assets/0dd16828-b7f2-47e6-9008-62b9aa000813" />
