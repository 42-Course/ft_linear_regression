# Linear Regression with Gradient Descent: Rust Edition 🚀

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)
![Contributions Welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg)
![Made With Love](https://img.shields.io/badge/made%20with-%E2%9D%A4-red.svg)

Welcome to **Linear Regression with Gradient Descent**, a Rust-based project designed to make linear regression approachable and visually stunning! Whether you’re a data science enthusiast or a Rustacean, this project has something for you.

***

![Screenshot from 2025-03-29 16-57-22](https://github.com/user-attachments/assets/b7a1fab6-a2e2-40bb-8d58-f0dd661d8bab)

![Screenshot from 2025-03-29 16-59-06](https://github.com/user-attachments/assets/97bb868d-70e5-4c5e-98ee-f59fdad4479b)

![Screencastfrom2025-02-0600-26-20-ezgif com-video-to-gif-converter](https://github.com/user-attachments/assets/84a2bd0e-9970-42ae-8b20-461139e0906f)


<details>
   <summary>Old version</summary>
   
![ScreencastFrom2024-12-3120-06-25-ezgif com-video-to-gif-converter](https://github.com/user-attachments/assets/3f521a91-2c56-40a4-a386-1f3a1815e65a)
</details>

---

## Project Overview

This repository implements a linear regression model using gradient descent. It includes:

1. **Trainer**: Train the model with your dataset.
2. **Predictor**: Use the trained model to predict values.
3. **GUI**: Visualize the data, regression line, and gradient descent process interactively.
4. **Linear Regression**: library to handle linear regression stuff.
---

## Features

- **Gradient Descent**: Train the model iteratively.
- **Interactive GUI**: Adjust settings, visualize plots, and watch gradient descent in action.
- **Makefile Integration**: Simplify build and execution workflows.
- **Environment Configuration**: Store paths and configurations in a `.env` file.

---

## Project Structure

Here’s the project layout:

```plaintext
.
├── Cargo.lock              # Dependency lock file
├── Cargo.toml              # Workspace configuration
├── data                    # Data folder
│   ├── data.csv            # Dataset for training
│   └── theta.txt           # Model parameters (theta0, theta1)
├── gui                     # GUI application
│   ├── Cargo.toml
│   └── src
│       ├── app.rs          # Main application logic
│       ├── components      # Reusable UI components
│       │   ├── mod.rs
│       │   ├── navbar.rs
│       │   ├── plane.rs
│       │   └── sidebar.rs
│       ├── main.rs         # GUI entry point
│       ├── settings        # Settings modules
│       │   ├── grid_settings.rs
│       │   ├── mod.rs
│       │   ├── plot_settings.rs
│       │   └── sidebar_settings.rs
│       └── utils.rs        # Helper functions
├── linear_regression       # Core library for regression logic
│   ├── Cargo.toml
│   └── src
│       ├── lib.rs
│       ├── linear_regression.rs
│       └── utils.rs
├── Makefile                # Build and execution shortcuts
├── predictor               # Predictor application
│   ├── Cargo.toml
│   └── src
│       └── main.rs         # Entry point for predictions
├── README.md               # This file!
├── trainer                 # Trainer application
│   ├── Cargo.toml
│   └── src
│       └── main.rs         # Entry point for training
```

---

## Environment Variables

To keep things clean and configurable, the project uses a `.env` file. Make sure to create it in the root directory:

```
THETA_PATH=data/theta.txt
DATASET_PATH=data/data.csv
```

### Explanation:
- **THETA_PATH**: Path to the file where model parameters (theta0, theta1) are stored.
- **DATASET_PATH**: Path to the CSV file containing your dataset.

---

## Makefile Commands

The `Makefile` simplifies building and running the project. Here’s what you can do:

### Build the Project
```bash
make build
```

### Run the Predictor
```bash
make predictor
```

### Run the Trainer
```bash
make trainer
```

### Run the GUI
```bash
make gui
```

### Run the GUI webapp
```bash
make web
```

### Clean the Workspace
```bash
make clean
```

---

## Example Usage

### Train the Model
1. Ensure `data/data.csv` contains your dataset (e.g., mileage vs. price).
2. Run the trainer:
   ```bash
   make trainer
   ```
3. The trained parameters will be saved to `data/theta.txt`.

### Predict Values
1. Use the predictor to estimate values based on the trained model:
   ```bash
   make predictor
   ```
2. Enter a mileage value when prompted, and the program will output the estimated price.

### Visualize with GUI
1. Run the GUI:
   ```bash
   make gui
   # or
   make web
   ```
2. Explore your dataset, adjust parameters, and watch the regression line and gradient descent come to life!

---

## Example Dataset

Your `data/data.csv` should look like this:
```csv
mileage,price
5000,20000
10000,18000
15000,16000
20000,14000
25000,12000
```

---

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to enhance the project.

---

Enjoy using **Linear Regression with Gradient Descent**! 🚀 Let me know if you have any feedback or feature requests.

