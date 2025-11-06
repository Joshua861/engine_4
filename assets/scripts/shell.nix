{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    python3
    python3Packages.pip
    python3Packages.virtualenv
  ];

  shellHook = ''
    # Create a virtual environment if it doesn't exist
    if [ ! -d .venv ]; then
      echo "Creating virtual environment..."
      python -m venv .venv
    fi

    # Activate the virtual environment
    source .venv/bin/activate

    # Install coloraide if not already installed
    if ! python -c "import coloraide" 2>/dev/null; then
      echo "Installing coloraide..."
      pip install coloraide
    fi

    echo "Virtual environment activated with coloraide installed"
    echo "Python: $(which python)"
  '';
}
