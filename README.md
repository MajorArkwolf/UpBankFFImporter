# UP Bank to FireFly III Data Importer
**This is still a work in progress**

This program is designed to import transactions from Up Bank Australia into Firefly 3 either as a once-off or continuously. It is also able to detect if a transaction has been updated such as a new tag being added or a category change in the Up Bank portal which will then update the transaction in Firefly to reflect this change. 

## Building
Building is simple, just ensure you have rust installed and run the following,
``cargo build``
or
``cargo build --release``

## Setup
- Copy the ``settings-template.yaml`` and rename it just ``settings.yaml``
- Get the Up Bank PAN code and paste it inside of the quotes in ``settings.yaml`` -> ``upbank_pan``
- Get the Firefly PAN code and paste it inside of the quotes in ``settings.yaml`` -> ``ff_pan``
- Add the url to your Firefly instance (eg 192.168.0.2:8083) into the ``settings.yaml``
- For each up bank account you wish to import into Firefly, do the following...
  - Get the unique GUID from UpBank for the account you wish to create and note it down.
    - Note: Passing the action "get-account-info" into the executable will print out all your Up Bank account information to the terminal which can be used to help set up your Firefly account information.
  - Create a new asset account inside of Firefly that represents the Up Bank Account you wish to import
  - Under the new asset account you created in Firefly, under ``Optional fields`` -> ``Account Number``, paste the GUID from Up Bank. 

### Docker-Compose.yml
If you are using the provided Docker-Compose.yml, after you have finished your first import, you should uncomment ``DATE_RANGE=30``, as this will considerably speed up the process by fetching less data.

## Running
Ensure you have setup your settings.yaml file before continuing
### Command Line Help
Linux & macOS : 
``up_bank_fidi -?``

Windows: ``up_bank_fidi.exe -?``

### Getting Up Bank Account Information
Linux & macOS : 
``up_bank_fidi get-account-information``

Windows: ``up_bank_fidi.exe get-account-information``

### Running the migrator tool
See the command line help assistant for setting start and end dates or date ranges.

Linux & macOS : 
``up_bank_fidi``

Windows: ``up_bank_fidi.exe``

## Docker
This program is best used from a docker container. Provided is both a Dockerfile and a template Docker-Compose.yml file.

### Docker Compose
Docker compose is setup to be built from a repo clone. You may also point your host volumes elsewhere if required.
