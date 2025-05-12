# Vérifier si cargo-zigbuild est installé
if (-not (Get-Command cargo-zigbuild -ErrorAction SilentlyContinue)) {
    Write-Host "cargo-zigbuild n'est pas installé. Installation en cours..."
    cargo install cargo-zigbuild
    if ($?) {
        Write-Host "cargo-zigbuild a ete installe avec succes."
    } else {
        Write-Host "Erreur lors de l'installation de cargo-zigbuild."
        exit 1
    }
} else {
    Write-Host "cargo-zigbuild est deja installe."
}

# Vérifier si le target x86_64-unknown-linux-gnu est ajouté
$targetList = rustup target list
if ($targetList -notcontains "x86_64-unknown-linux-gnu") {
    Write-Host "Target x86_64-unknown-linux-gnu n'est pas ajoute. Ajout en cours..."
    rustup target add x86_64-unknown-linux-gnu
    if ($?) {
        Write-Host "Target x86_64-unknown-linux-gnu a ete ajoute avec succes."
    } else {
        Write-Host "Erreur lors de l'ajout du target x86_64-unknown-linux-gnu."
        exit 1
    }
} else {
    Write-Host "Target x86_64-unknown-linux-gnu est deja ajoute."
}

# Compiler pour Linux
Write-Host "Compilation pour Linux..."
cargo zigbuild --target x86_64-unknown-linux-gnu --features "linux"
if ($?) {
    Write-Host "Compilation terminée pour Linux!"
} else {
    Write-Host "Erreur lors de la compilation. Vérifiez les logs ci-dessus."
    exit 1
}