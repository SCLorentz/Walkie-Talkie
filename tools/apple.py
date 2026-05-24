# Generated with interaction with Gemni 3.5 flash

import os
import shutil
import subprocess

def get_binary_name():
    """Lê o Cargo.toml manualmente para extrair o nome do binário."""
    try:
        with open("Cargo.toml", "r", encoding="utf-8") as f:
            in_package_section = False
            for line in f:
                line = line.strip()
                # Ignora linhas vazias ou comentários
                if not line or line.startswith("#"):
                    continue
                # Identifica se entrou na seção [package]
                if line.startswith("[") and line.endswith("]"):
                    in_package_section = (line == "[package]")
                    continue
                # Se estiver dentro de [package], procura pela chave name
                if in_package_section and line.startswith("name"):
                    # Extrai o valor entre aspas (ex: name = "meu_app" -> meu_app)
                    parts = line.split("=", 1)
                    if len(parts) == 2:
                        return parts[1].strip().strip('"').strip("'")
        print("Erro: Chave 'name' não encontrada na seção [package] do Cargo.toml.")
        return None
    except FileNotFoundError:
        print("Erro: Arquivo Cargo.toml não encontrado no diretório atual.")
        return None

def create_macos_app_bundle(output_dir="."):
    """Cria a estrutura do .app, compila o projeto Rust e copia os arquivos."""
    # 1. Obter nome do app pelo Cargo.toml
    app_name = get_binary_name()
    if not app_name:
        return

    # 2. Executar o Cargo Build
    print("Executando 'cargo build --release'...")
    result = subprocess.run(["cargo", "build", "--release"], check=False)
    if result.returncode != 0:
        print("Erro: A compilação do Cargo falhou.")
        return

    # Define os caminhos
    script_dir = os.path.dirname(os.path.abspath(__file__))
    source_plist = os.path.join(script_dir, "Info.plist")
    source_icon = os.path.join(script_dir, "icon.icns")
    source_binary = os.path.join(".", "target", "release", app_name)

    app_path = os.path.join(output_dir, f"{app_name}.app")
    contents_path = os.path.join(app_path, "Contents")
    macos_path = os.path.join(contents_path, "MacOS")
    resources_path = os.path.join(contents_path, "Resources")
    frameworks_path = os.path.join(contents_path, "Frameworks")

    # Validações de arquivos de origem
    if not os.path.exists(source_plist):
        print(f"Erro: O arquivo {source_plist} não foi encontrado.")
        return
    if not os.path.exists(source_icon):
        print(f"Erro: O arquivo de ícone {source_icon} não foi encontrado.")
        return
    if not os.path.exists(source_binary):
        print(f"Erro: O binário compilado {source_binary} não foi encontrado.")
        return

    # 3. Cria a estrutura de diretórios
    for path in [macos_path, resources_path, frameworks_path]:
        os.makedirs(path, exist_ok=True)

    # 4. Copia o Info.plist
    destination_plist = os.path.join(contents_path, "apple/Info.plist")
    shutil.copy2(source_plist, destination_plist)
    print("Copiado: Info.plist")

    # 5. Copia o arquivo de ícone para a pasta Resources
    destination_icon = os.path.join(resources_path, "apple/icon.icns")
    shutil.copy2(source_icon, destination_icon)
    print("Copiado: icon.icns")

    # 6. Copia o binário e garante permissão de execução
    destination_binary = os.path.join(macos_path, app_name)
    shutil.copy2(source_binary, destination_binary)
    os.chmod(destination_binary, 0o755)  # Permissão rwxr-xr-x
    print(f"Copiado e configurado binário: {app_name}")

    print(f"\nSucesso! Seu pacote {app_name}.app está pronto em: {app_path}")

if __name__ == "__main__":
    create_macos_app_bundle()
