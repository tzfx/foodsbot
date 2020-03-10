# Foodsby and Slack

## Pre-reqs
Install the following:
- openssl-devel
- rustup <https://www.rust-lang.org/tools/install>

## Configuration
Global Variables to set:
- FOODSBOT_SLACK_TOKEN: Slack OAUTH token.
- FOODSBOT_CHANNEL: The slack channel name to post in.
- FOODSBOT_LOCATION: The kiosk location id to pull the foodsby restaurant menu from.

## Building
build with cargo build

## Running
do whatever you want with the binary.
A cron job or systemd file seems right.