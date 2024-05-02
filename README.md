[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-24ddc0f5d75046c5622901739e7c5dd533143b0c8e959d652212380cedb1ea36.svg)](https://classroom.github.com/a/44KrvlFZ)
# CS-370 Project 4: Hanging on by a Thread
Authors: Thomas Kay & Dylan Miller

Professor: Dr. William Kreahling

Submission Date: May 3rd, 2024

## Overview 
This project models a supply chain where:
 - Depot: Acts as a central storage for resources, managing the stock and availability of three resources: Burnstone, Seaplum, and Klah.
 - Strongholds: Consume resources to produce output, and signal when resources are needed and when they have been used.
 - Dragonriders: Deliver resources from the depot to the strongholds, helping keep the supply chain functioning smoothly.

The system employs synchronization mechanisms like Mutex and Condvar to manage inter-thread communication and synchronization, ensuring a coherent flow between depots, strongholds, and dragonriders.

## Usage
Running the simulation:

    cargo run <time_limit> <log_mode>

Where:
 - <time_limit> specifies the runtime duration in seconds (e.g., 60 for a one-minute runtime). This value controls how long the simulation will run before it terminates.
 - <log_mode> determines the logging mode: use 'T' to log to a file named log.txt, or 'F' to print logging information directly to the console.

For example:

    cargo run 60 F

This runs the simulation for 60 seconds and logs output directly to the console.
