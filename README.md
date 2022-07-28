# UP Bank to FireFly III Data Importer
This is still a work in progress

## Setup
- Copy the settings-template.yaml and rename it just settings.yaml
- Add the url to your Firefly instance (eg 192.168.0.2:8083)
- For each account you wish to import data from do the following
  - Create a firefly account that represents your Up Bank Account
  - In firefly add the unique id from Up Bank into the Firefly Account Number field. (This creates a direct link from the importer tool)
  - Note: Passing the action "get-account-info" will print out all your Up Bank account information to the terminal which can be used to help setup your Firefly account information.
  - Add the Upbank id into the accounts section in the settings.yaml, this tells the importer to only import this data.