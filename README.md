# UP Bank to FireFly III Data Importer
**This is still a work in progress**

This program is designed to import transaction from Up Bank Australia into Firefly 3 either as a once off or continously. It is also able to detect if a transaction has been updated such as a new tag is added or category change in the Up Bank portal and update the transaction in Firefly to reflect this change. 

## Setup
- Copy the settings-template.yaml and rename it just settings.yaml
- Add the url to your Firefly instance (eg 192.168.0.2:8083)
- For each account you wish to import data from do the following
  - Create a firefly account that represents your Up Bank Account
  - In firefly add the unique id from Up Bank into the Firefly Account Number field. (This creates a direct link from the importer tool)
  - Note: Passing the action "get-account-info" will print out all your Up Bank account information to the terminal which can be used to help setup your Firefly account information.
  - Add the Upbank id into the accounts section in the settings.yaml, this tells the importer to only import this data.

## Docker
This program is best used from a docker container. Provided is both a Dockerfile and a template Docker-Compose.yml file.

### Docker Compose
Docker compose is setup to be built from a repo clone. You may also point your host volumes elsewhere if required.