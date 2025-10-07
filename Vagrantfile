# Vagrant configuration for headless GUI testing (VMware Fusion provider)
Vagrant.configure("2") do |config|
  config.vm.box = "generic/ubuntu2204"
  config.vm.provider :vmware_fusion do |v|
    v.gui = false
    v.vmx["memsize"] = 4096
    v.vmx["numvcpus"] = 2
  end

  config.vm.synced_folder ".", "/workspace", type: "rsync", rsync__args: ["--verbose", "--archive", "--delete"]

  config.vm.provision "shell", inline: <<-SHELL
    set -e
    sudo apt-get update
    sudo apt-get install -y python3 python3-pip xvfb
    sudo apt-get install -y libgtk-3-0 libx11-xcb1 libxcb-randr0 libxcb-shm0 libxcb-xfixes0 libxcb1-dev
    if ! command -v rustup >/dev/null; then
      curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path
    fi
    source $HOME/.cargo/env
    cd /workspace/rust/hash-checker
    cargo test
  SHELL
end
