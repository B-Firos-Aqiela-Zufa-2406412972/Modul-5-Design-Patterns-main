# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

1. jika semua subscriber memiliki perilaku yang sama, maka sebuah Model struct tunggal sudah cukup. Namun, jika ke kita ingin menggunakan berbagai macam subscriber, maka penggunaan trait menjadi penting agar sistem tetap decoupled dan fleksibel.
2. Map secara alami menjamin bahwa satu kunci hanya merujuk pada satu nilai. Jika menggunakan Vec kita harus melakukan pengecekan manual setiap kali menambah data agar tidak ada duplikasi. 
3. meskipun kita menggunakan pola Singleton untuk menyimpan daftar subscriber, kita tetap membutuhkan DashMap. Hal ini dikarenakan compiler Rust tidak akan mengizinkan kita mengubah variabel statis dari banyak thread secara bersamaan tanpa pembungkus yang aman secara thread.

#### Reflection Publisher-2

1. Pemisahan antara Service, Repository, dan Model bertujuan untuk menerapkan prinsip Single Responsibility Principle. Dengan pemisahan ini, Model hanya berfungsi sebagai representasi data, sementara Repository bertanggung jawab penuh atas logika akses dan penyimpanan data. Di sisi lain, Service berperan untuk mengelola logika bisnis dan mengoordinasikan interaksi antar komponen.

2. Jika seluruh logika bisnis dan penyimpanan digabungkan langsung ke dalam Model, maka akan terjadi tingkat ketergantungan yang sangat tinggi. Model Subscriber tidak hanya akan menyimpan data, tetapi juga harus memahami cara mengelola dirinya sendiri dalam, sementara model Notification akan terbebani dengan detail pengiriman dan pengambilan data subscriber. Keterikatan antar model ini membuat kode sulit untuk dimodifikasi karena perubahan kecil pada satu bagian dapat merusak logika di bagian lain, sehingga sistem menjadi sulit untuk dikembangkan.

3. Postman merupakan alat yang sangat esensial dalam pengembangan API karena memungkinkan pengujian fungsionalitas backend secara mandiri tanpa harus menunggu kesiapan sisi frontend.
#### Reflection Publisher-3
1. kita menggunakan variasi Push model. Hal ini terlihat dari metode update pada Subscriber dan metode notify pada NotificationService. Ketika terjadi perubahan status pada produk Publisher akan membungkus semua detail data tersebut ke dalam objek Notification dan langsung dipush ke setiap Subscribe

2. Jika sistem ini diubah menggunakan Pull model 
- Kelebihan: Model lebih efisien dalam penggunaan bandwidth dan memori jika Subscriber hanya peduli pada sebagian kecil data, atau jika Subscriber sedang sibuk dan lebih memilih untuk mengambil data nanti saat mereka siap. Subscriber memegang kendali penuh atas kapan dan apa yang ingin mereka ambil.
- Kekurangan: Akan terjadi peningkatan latensi dan beban jaringan. Sistem harus melakukan dua kali perjalanan: pertama, Publisher mengirimkan sinyal notifikasi, lalu kedua, Subscriber membalas dengan HTTP GET request ke Publisher untuk mengambil data spesifiknya. Jika jumlah subscriber sangat banyak dan mereka semua "menarik" data secara bersamaan server BambangShop bisa mengalami kelebihan beban.

3. program akan berjalan secara synchronous. Proses thread utama akan terblokir setiap kali mengirim notifikasi ke satu Subscriber dan harus menunggu hingga HTTP request tersebut selesai atau timeout sebelum bisa lanjut mengirim ke Subscriber berikutnya. Akhirnya waktu respons dari endpoint produk akan menjadi sangat lambat.
