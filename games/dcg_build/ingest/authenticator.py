import os
import pickle

from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow


class Authenticator:
    # These are the permissions the script will ask for.
    # If modifying these scopes, delete the file token.pickle
    # and rerun the script.
    SCOPES = [
        "https://www.googleapis.com/auth/drive.metadata.readonly",
        "https://www.googleapis.com/auth/spreadsheets.readonly",
    ]

    @staticmethod
    def get_credentials() -> Credentials:
        """
        Authenticates the user and sets up their credentials to download
        information from their Google Drive
        """
        creds = None

        # The file token.pickle stores the user's access and refresh tokens,
        # and is created automatically when the authorization flow completes
        # for the first time.
        if os.path.exists("token.pickle"):
            with open("token.pickle", "rb") as token:
                creds = pickle.load(token)

        # If there are no (valid) credentials available, let the user log in.
        if not creds or not creds.valid:
            if creds and creds.expired and creds.refresh_token:
                creds.refresh(Request())
            else:
                try:
                    flow = InstalledAppFlow.from_client_secrets_file(
                        r"client_secrets.json", Authenticator.SCOPES
                    )
                    creds = flow.run_local_server(port=0)
                except FileNotFoundError:
                    print(
                        "\nERROR: Couldn't find 'client_secrets.json'\n"
                        "Make sure you have downloaded your credentials from "
                        "Google Cloud console!\n\n"
                        "You must be added to this project:\n"
                        "https://console.cloud.google.com/apis/dashboard?"
                        "authuser=1&project=mercurial-snow-294416\n\n"
                        "And you can find credentials here:\n"
                        "https://console.cloud.google.com/apis/credentials?"
                        "authuser=1&project=mercurial-snow-294416"
                    )
                    exit(1)
            # Save the credentials for the next run
            with open("token.pickle", "wb") as token:
                pickle.dump(creds, token)

        return creds
