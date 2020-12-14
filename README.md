# Getting Started

These steps will get this sample application in Rust running for you using DigitalOcean.

**Note: Following these steps will result in charges for the use of DigitalOcean services.**

**See below to delete an app once it is no longer needed**


## How to Deploy this Sample App (without forking the Repo or using it as a Template)

Click the button below to deploy the app to the DigitalOcean App Platform. If you don't have a DigitalOcean account yet, don't worry! After clicking the button you'll be walked through our account creation process.

 [![Deploy to DO](https://mp-assets1.sfo2.digitaloceanspaces.com/deploy-to-do/do-btn-blue-ghost.svg)](https://cloud.digitalocean.com/apps/new?repo=https://github.com/davidedelpapa/do_apps_rust_example/tree/main&refcode=8080bc0cceab)
 
If you use the above button, you will get also a $100 in credit in DigitalOcean products over 60 days.
This way, you can take a quick peek at the app creation process, before going over the tutorial.

The procedure is guided, and you can just get the defaults, however upon choosing the plan, on the bottom of the page, you can select the smallest environment (the *512 MB RAM, 1 CPU* container). It might take even a quarter of an hour to build, but as of speed of execution it is the same (Rocket is not taxing on resources).

If you plan to make changes to this app, after trying it, continue to the next section.

## How to Deploy this Sample App in your own Repo

To use all the features of App Platform, you need to be running against your own copy of this application. You can either:

1. click the Fork button near the top right of this window and follow the on-screen instructions. 
2. or: use the `use this template` green button above.

## Follow the Tutorial

[Follow this tutorial](https://dev.to/davidedelpapa/digital-ocean-app-platform-hackathon-using-not-natively-supported-platforms-rust-rocket-55og) to make your own app at [dev.to](https://dev.to/davidedelpapa/)

## Deleting the App

When you no longer need this sample application running live, you can delete it by following these steps:
1. Visit the Apps control panel at https://cloud.digitalocean.com/apps
1. Navigate to the do-apps-rust-example app
1. Choose "Settings"->"Destroy"

This will delete the app and destroy any underlying DigitalOcean resources

**Note: If you don't delete your app, charges for the use of DigitalOcean services will continue to accrue.**

## Learn More about DigitalOcean App Platform

You can learn more about the App Platform and how to manage and update your application at https://www.digitalocean.com/docs/app-platform/.
