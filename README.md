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
-   [x] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Subscriber model struct.`
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [x] Commit: `Implement add function in Subscriber repository.`
    -   [x] Commit: `Implement list_all function in Subscriber repository.`
    -   [x] Commit: `Implement delete function in Subscriber repository.`
    -   [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [x] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [x] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [x] Commit: `Implement publish function in Program service and Program controller.`
    -   [x] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [x] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. In the Observer design pattern, choosing between an interface (or a trait in Rust) and a single Model struct for the BambangShop application depends on how simple or complex the system needs to be. If every subscriber to updates behaves the same way, then just one Model struct could be enough, which keeps things simple. However, if BambangShop might need to handle different kinds of subscribers later on, each reacting differently, it's better to use a trait. This makes the system more flexible and easier to update because new kinds of subscribers can be added without changing how the main system works. Using a trait helps keep different parts of the system separate, which is a key goal of the Observer pattern. This setup makes it easier to manage and extend the system as it grows.
2. Using a Vec (a type of list) to store unique IDs and URLs in the BambangShop application would mean you have to check every time you add a new one to make sure it's not already there, which can be slow and easy to mess up, especially if the list gets long. Instead, using a DashMap, which is a special type of dictionary in Rust, is a better choice. DashMap automatically prevents adding duplicates and lets you quickly find and update entries. It can also handle many users adding or changing data at the same time without slowing down, which is great for busy apps. So, for managing unique IDs and URLs efficiently and safely, DashMap is the way to go instead of a Vec.
3. In Rust programming, making sure programs are safe to run with multiple threads at the same time is very important. We use DashMap for storing a list of subscribers because it's specifically designed to handle multiple threads safely and efficiently. Although we could use the Singleton pattern, which ensures that only one instance of a class is created, it doesn't automatically handle thread safety. This means that if we used Singleton, we'd need extra steps to make sure it's safe when multiple threads access it, which can make things more complicated. DashMap, on the other hand, is built to be thread-safe right from the start, which makes it a better choice for handling data that multiple threads need to access simultaneously in Rust.

#### Reflection Publisher-2
1. In the Model-View-Controller (MVC) architecture, it's good to separate Service and Repository from the Model to make the system easier to manage, update, and test. This is based on the Single Responsibility Principle, which suggests that each part of the system should handle just one specific task. By separating them, changes in one area won't interfere with another part. This not only makes the system simpler to maintain but also makes it easier to test each component separately, ensuring that each part is working correctly without affecting others.
2. If we use only the Model in an MVC architecture to handle everything, it could make the system very complicated and hard to maintain. Imagine having different models like Product, Subscriber, and Notification, each supposed to handle specific tasks. If these models also take on additional responsibilities, such as managing data access or business logic, they become overly complex. This is because each model would need to know too much about how other parts of the system work, leading to a higher chance of mistakes.
3. Postman is a really useful tool for my software projects. It helps me check my work by making data easy to read, no matter what format it's in. This is especially handy when I'm working with others because it ensures our data is accurate. Postman also lets me test different parts of my project, like sending requests and seeing how things respond. 

#### Reflection Publisher-3
1. The Observer Pattern is implemented using the Push model. The publisher actively sends data to the subscribers when an event of interest is triggered. Specifically, this occurs through the NotificationService's notify method, where each subscriber's update method is directly called with the notification payload. Unlike the Pull model, where users have to ask for updates, in the Push model the system takes the initiative to send information directly to users without them needing to request it.
2. Switching to the Pull model in the tutorial scenario has both upsides and downsides. Advantages include less strain on the server since updates are only sent when subscribers ask for them, and subscribers have more control over when they get updates, which can help avoid overload. Disadvantages involve more network traffic and potential delays because subscribers have to check for updates themselves, which might lead to frequent unnecessary checks or missing timely information when updates are not checked regularly.
3. If we decide not to use multi-threading in the notification process of our program, the notifications would be sent sequentially, one after the other. This could lead to several issues, especially if there are many subscribers or if some subscribers are slow to process notifications. Additionally, this sequential processing could slow down or even temporarily freeze the entire application, because it would be tied up waiting for each notification process to finish before moving on to the next task.
