# MUA Connect

Platform marketplace yang menghubungkan calon pelanggan dengan Makeup Artist (MUA) profesional di seluruh Indonesia.

## Struktur Proyek

```
mua-connect/
├── backend/          # Rust API dengan Actix-web
├── frontend/         # React aplikasi dengan Vite
└── readme.md         # Dokumentasi PRD
```

# MUA Connect - Product Requirements Document (PRD) Summary

## 1. Deskripsi Proyek

MUA Connect adalah aplikasi *full-stack* yang dirancang sebagai platform *marketplace* yang menghubungkan calon pelanggan dengan *Makeup Artist* (MUA) profesional di seluruh Indonesia. Aplikasi ini bertujuan untuk memecahkan masalah inefisiensi dan fragmentasi di pasar jasa kecantikan dengan menyediakan seperangkat alat manajemen bisnis yang komprehensif bagi MUA dan proses penemuan serta pemesanan yang mulus bagi pelanggan.

**Backend:** Rust (API)
**Frontend:** React

---

## 2. Visi Produk

"Menjadi platform digital definitif untuk industri jasa kecantikan di Indonesia, memberdayakan para *makeup artist* untuk membangun karier yang berkelanjutan dengan menyediakan alat manajemen bisnis profesional, dan memungkinkan pelanggan untuk menemukan serta memesan para profesional kecantikan tepercaya secara mudah dan penuh keyakinan."

---

## 3. Masalah yang Diselesaikan

*   **Untuk Pelanggan:** Proses menemukan MUA yang berkualitas dan tersedia sangat rumit, tidak efisien, dan penuh ketidakpastian. Pelanggan harus mencari secara manual di media sosial, kesulitan memverifikasi kualitas, dan melalui proses komunikasi yang panjang hanya untuk menanyakan harga dan ketersediaan.
*   **Untuk MUA:** Para MUA, terutama pemula, kesulitan dalam pemasaran dan akuisisi klien yang konsisten. Mereka menghabiskan terlalu banyak waktu untuk tugas-tugas administratif (menjawab pertanyaan berulang, penjadwalan, penagihan) daripada fokus pada keahlian mereka. Hal ini menyebabkan persaingan tidak sehat dan kesulitan membangun bisnis yang berkelanjutan.

---

## 4. Persona Pengguna Target

### a. Pelanggan: "Amelia, si Peserta Acara"
*   **Profil:** Profesional muda (28 tahun) yang aktif secara sosial dan membutuhkan jasa MUA untuk acara-acara penting.
*   **Tujuan:** Menemukan MUA berbakat yang sesuai dengan gayanya, memesan dengan cepat, dan merasa yakin dengan kualitas layanan.
*   **Frustrasi:** Kesulitan mencari di tengah banjir informasi, ketidakpastian kualitas MUA, proses pemesanan yang manual dan lambat, serta harga yang tidak transparan.

### b. MUA: "Rina, si Profesional yang Berkembang"
*   **Profil:** MUA lepas (24 tahun) yang bersemangat dan ambisius untuk mengembangkan bisnisnya.
*   **Tujuan:** Meningkatkan jumlah klien, membangun merek profesional, dan menyederhanakan tugas administratif agar bisa fokus pada seni merias.
*   **Frustrasi:** Beban pemasaran di media sosial, kelelahan administratif karena menjawab pertanyaan yang sama berulang kali, dan risiko kesalahan manajemen seperti pemesanan ganda (*double booking*).

---

## 5. Fitur Inti (Minimum Viable Product - MVP)

Berdasarkan kerangka kerja MoSCoW, berikut adalah fitur-fitur yang **wajib ada** untuk peluncuran awal:

*   **Pembuatan Akun Pengguna:** Proses pendaftaran dan login untuk Pelanggan dan MUA.
*   **Manajemen Portofolio MUA:** MUA dapat mengunggah foto karya, membuat daftar layanan, deskripsi, dan harga.
*   **Pencarian & Penemuan Dasar:** Pelanggan dapat mencari MUA berdasarkan lokasi dan tanggal.
*   **Manajemen Kalender & Ketersediaan:** MUA dapat mengatur jadwal kerja dan memblokir waktu yang tidak tersedia untuk mencegah pemesanan ganda.
*   **Alur Pemesanan & Persetujuan:** Pelanggan dapat mengajukan permintaan pemesanan, dan MUA dapat menyetujui atau menolaknya.
*   **Obrolan Dalam Aplikasi (*In-App Chat*):** Fitur komunikasi *real-time* antara pelanggan dan MUA untuk konsultasi sebelum memesan.
*   **Sistem Pembayaran:** Integrasi gerbang pembayaran untuk menangani uang muka (*deposit*) dan pembayaran akhir secara aman.
*   **Sistem Peringkat & Ulasan:** Pelanggan dan MUA dapat saling memberikan ulasan setelah transaksi selesai untuk membangun kepercayaan.

---

## 6. Tumpukan Teknologi & Arsitektur

### a. Backend (Rust)
*   **Kerangka Kerja:** API RESTful akan dikembangkan menggunakan **Actix-web** atau **Axum** karena performa tinggi dan fitur asinkron.
*   **Basis Data:** **PostgreSQL** dipilih karena integritas transaksionalnya yang kuat, penting untuk sistem pemesanan.
*   **Desain API:** Mengikuti prinsip RESTful dengan dokumentasi menggunakan spesifikasi OpenAPI.

### b. Frontend (React)
*   **Arsitektur:** Arsitektur berbasis komponen dengan pemisahan antara komponen presentasional (UI) dan kontainer (logika).
*   **Struktur Folder:** Berbasis fitur (*feature-first*) untuk skalabilitas dan kemudahan pemeliharaan.
*   **Manajemen State:**
    *   **Server State:** Menggunakan **React Query** atau **SWR** untuk *caching*, sinkronisasi, dan pengambilan data dari API.
    *   **Global UI State:** Menggunakan **Zustand** atau **Redux Toolkit** untuk state global seperti status otentikasi.
    *   **Local Component State:** Menggunakan *hooks* bawaan React (`useState`, `useReducer`).

---

## 7. Peta Jalan Pasca-Peluncuran

*   **V1.1 (Fokus Pemberdayaan MUA):** Dasbor analitik lanjutan, integrasi kalender eksternal, dan alat promosi untuk MUA.
*   **V1.2 (Fokus Pengalaman Pelanggan):** Filter pencarian lanjutan, fitur "Favorit", dan notifikasi pengingat janji temu.
*   **V2.0 (Pertumbuhan Ekosistem):** Ekspansi ke layanan kecantikan lain, model langganan premium untuk MUA, dan konten edukasi (blog/tutorial).

## Development

### Backend
```bash
cd backend
cargo run
```

### Frontend
```bash
cd frontend
npm install
npm run dev
```