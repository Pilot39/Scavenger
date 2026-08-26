{{- with secret "secret/data/scavenger/api" -}}
VITE_CONTRACT_ID={{ .Data.data.contract_id }}
VITE_NETWORK={{ .Data.data.network }}
VITE_RPC_URL={{ .Data.data.rpc_url }}
VITE_FIREBASE_API_KEY={{ .Data.data.firebase_api_key }}
VITE_FIREBASE_AUTH_DOMAIN={{ .Data.data.firebase_auth_domain }}
VITE_FIREBASE_PROJECT_ID={{ .Data.data.firebase_project_id }}
VITE_FIREBASE_STORAGE_BUCKET={{ .Data.data.firebase_storage_bucket }}
VITE_FIREBASE_MESSAGING_SENDER_ID={{ .Data.data.firebase_messaging_sender_id }}
VITE_FIREBASE_APP_ID={{ .Data.data.firebase_app_id }}
VITE_FIREBASE_MEASUREMENT_ID={{ .Data.data.firebase_measurement_id }}
{{- end }}

{{- with secret "secret/data/scavenger/stellar" -}}
STELLAR_SECRET_KEY={{ .Data.data.secret_key }}
STELLAR_NETWORK={{ .Data.data.network }}
{{- end }}
