#!/bin/bash

# Demander si le dépôt doit être privé
while true; do
  read -p "Le dépôt doit-il être privé ? ([t]rue/[f]alse): " PRIVATE

  case "$PRIVATE" in
    t|T) PRIVATE="true"; break ;;
    f|F) PRIVATE="false"; break ;;
    *) echo "Erreur : veuillez entrer 't' pour true ou 'f' pour false." ;;
  esac
done

TOKEN=""  # Remplacez par votre token GitHub
REPO_NAME=$1  # Le nom du dépôt
DESCRIPTION=""  # Description du dépôt
PRIVATE=$PRIVATE  # true pour un dépôt privé, false pour un dépôt public

# URL de l'API GitHub pour la création de dépôt
URL="https://api.github.com/user/repos"

# Créer le corps de la requête en format JSON
JSON_PAYLOAD=$(jq -n \
  --arg name "$REPO_NAME" \
  --arg description "$DESCRIPTION" \
  --argjson private "$PRIVATE" \
  '{name: $name, description: $description, private: $private}')

# Effectuer la requête POST pour créer le dépôt
response=$(curl -s -w "%{http_code}" -o /tmp/git_response.json \
  -X POST -H "Authorization: Bearer $TOKEN" \
  -H "User-Agent: Bash Script" \
  -d "$JSON_PAYLOAD" "$URL")

# Vérifier la réponse
if [ "$response" -eq 201 ]; then
  echo "Le dépôt '$REPO_NAME' a été créé avec succès sur GitHub."
else
  echo "Erreur lors de la création du dépôt : $(cat /tmp/git_response.json)"
fi

# Initialisation Git locale
git init
git add .
git commit -m "Initial commit"
git branch -M main

# Lier au dépôt distant et push
git remote remove origin 2>/dev/null
git remote add origin "https://github.com/Spiritecho777/$REPO_NAME.git"
git push -u origin main