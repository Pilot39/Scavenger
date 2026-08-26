vault {
  address = "http://vault:8200"
}

auto_auth {
  method {
    type = "kubernetes"
    config = {
      role = "scavenger-app"
    }
  }

  sink {
    type = "file"
    config = {
      path = "/tmp/vault-token"
      mode = 0640
    }
  }
}

cache {
  use_auto_auth_token = true
}

listener "unix" {
  address = "/tmp/vault.sock"
  tls_disable = true
}

listener "tcp" {
  address = "127.0.0.1:8100"
  tls_disable = true
}

template {
  source = "/etc/vault/templates/env.tpl"
  destination = "/app/.env"
  command = "systemctl restart scavenger"
}
