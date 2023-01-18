# BRAIIN CLOCK

A Linux CLI tool that allows you to broadcast current Braiins Pool bitcoin mining info from your user profile directly to your BlockClock Mini (https://blockclockmini.com/)

![thsblockclock](https://user-images.githubusercontent.com/55212954/158614788-8b850940-fb42-4c6b-ae84-7055e81db1b9.jpg)

**Display the following tags:**

1. Confirmed Reward
2. Unconfirmed Rewards
3. Estimated Reward
4. Alltime Reward
5. Hashrate 5m
6. Hashrate 60m
7. Hashrate 24h
8. Hashrate Scoring
9. Active Workers
10. Offline Workers
11. Estimate Hash Rate
12. USD Market Price
13. EUR Market Price
14. GBP Market Price
15. Sats per Dollar
16. Mempool Transactions
17. Difficulty Retarget Date
18. Blockchain Height
19. Moscow Time

*Got requests for other tags? Open an issue.*

----------------------

## Setup:

First you'll want to obtain your **Blockclock's IP** address as well as a **Braiins Auth Token**.

### Getting your Blockclock IP address:

This assume's that you have already setup your Blockclock Mini. 

If you need your Blockclock's IP, press on the second button from the top on the right side of the Blockclock. You'll see an IP address, likely in the 5th square.

We'll need to set your blockclock to manual in order to send it GET requests. Enter your clock's IP in a browser

You now have access to your Blockclock's settings page. On the **Display Page**, go down to **Display Preferences**, set **Screen Update Rate** to **Manual**.

That is all! 

### Getting your Braiins Auth Token:

Login:
https://pool.braiins.com/login

1. Click on the icon on the right of your username in the top right corner.
2. Click on **Devices**
3. Click on **Access Profiles**
4. Click on **Create New**
5. Add a username. **Access Permissions** can be set to **read-only**. 
   Check-off **Allow access to web APIs**, click **Generate new token** and copy that **Auth Token**. 
   Finally, click on **Create Access Profile**.

**Keep that IP address and Auth Token handy. You'll be asked to enter those on startup**

## Installation

The binary for braiin clock is braiin_clock 

Currently the only way to install the binary is with cargo. 

First install the rust toolchain - If you’re using Linux or macOS, open a terminal and enter the following command:

```curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh``` 

For Windows users visit: https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup

You'll need to download and run the rustup-init executable. 

Once install you can install the binary using ```cargo install```: 

```cargo install braiin_clock```

## Running the binary

Within a terminal window enter: ```braiin_clock```

The program will start. 

*If this command does not work make sure the ```.cargo/``` directory is on your Path.*


## How to use

You'll be asked to enter you Braiins API Token and your Blockclock's IP address.

Follow the instructions --> Select the tags you want to display and press "q" once you've selected your desired tags. Select the blockclock display refresh rate. If you want to change any of these settings you'll want to press CTRL+C to exit the program and start over.

The Blockclock will display the tags in the selected order, continuously, until you exit the program, close the terminal or shutdown your computer. 

This is a work in progress and there may be bugs. Please open an issue if you run into any. Thank you :D 
