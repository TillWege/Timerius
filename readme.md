# Timerius
A simple commandline timer software written in Rust that send you desktop-notifications

# Features
- Set custom timer duration
- Repeating timer
- Native OS notifications
- Low CPU usage

# How to install
With rust and cargo installed, you can install Timerius by running

``cargo install Timerius``

Alternatively you can install the current head of this repository with

``cargo install --git https://github.com/TillWege/Timerius.git``

After that you should be able to invoke the binary simply by running ``Timerius`` in your terminal of choice

# How to use
You can get a list of all available Commands by running

``Timerius -h``

To get information on any specific subcommand you can run

``Timerius <subcommand> -h``

You can start out by adding an alert by running

``Timerius add -n Notifcation -d "this is an alert from timerius" -i 60 -r``

After that you can start all of your timers by running 

``Timerius start``