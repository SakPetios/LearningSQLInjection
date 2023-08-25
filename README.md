# LearningSQLInjection


## Description

This repository contains a small web application that is intentionally designed to be vulnerable to SQL injection attacks. It's written in Rust using the Rocket framework. This project is intended for educational purposes and serves as a way to understand and learn about the risks associated with SQL injection vulnerabilities.

## Features

- Basic user authentication and login system.
- SQL injection vulnerability intentionally left in the codebase for educational purposes.
- Clear demonstration of how improper handling of user input can lead to security vulnerabilities.

## Setup and Usage

1. Clone the repository to your local machine.
2. Navigate to the project directory.
3. Install Rust and Cargo if not already installed.
4. Run the following command to start the application:
   ```shell
   cargo run
   ```
5. Access the website through your web browser at `http://localhost:8000`.

## Usage Example

1. Visit the login page on the web application.
2. Enter the following in the username field: `admin' --`.
3. Leave the password field empty.
4. Attempt to log in.

## Security Note

**CAUTION:** This application intentionally includes a security vulnerability that can be exploited using SQL injection. Do not deploy this code in a production environment or on the public internet. It is solely for educational purposes.

