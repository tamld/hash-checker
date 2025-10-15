# Vagrant configuration for headless GUI testing (defaults to VMware Fusion)
Vagrant.configure("2") do |config|
  config.vm.box = "generic/ubuntu2204"
  provider = ENV.fetch("VAGRANT_DEFAULT_PROVIDER", "vmware_fusion")

  case provider
  when "vmware_fusion"
    config.vm.provider :vmware_fusion do |v|
      v.gui = false
      v.vmx["memsize"] = 4096
      v.vmx["numvcpus"] = 2
    end
  when "vmware_desktop"
    config.vm.provider :vmware_desktop do |v|
      v.gui = false
      v.vmx["memsize"] = 4096
      v.vmx["numvcpus"] = 2
    end
  else
    warn "Using fallback provider '#{provider}'. Ensure it is installed (set VAGRANT_DEFAULT_PROVIDER to override)."
    config.vm.provider provider.to_sym do |v|
      v.gui = false if v.respond_to?(:gui=)
    end
  end

  config.vm.synced_folder ".", "/workspace", type: "rsync", rsync__args: ["--verbose", "--archive", "--delete"]

  config.vm.provision "shell", inline: <<-SHELL
    set -e
    sudo apt-get update
    sudo apt-get install -y xvfb pkg-config
    sudo apt-get install -y libgtk-3-dev libgtk-3-0 libx11-xcb1 libxcb-randr0 libxcb-shm0 libxcb-xfixes0 libxcb1-dev
    if ! command -v rustup >/dev/null; then
      curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path
    fi
    source $HOME/.cargo/env
    cd /workspace/rust/hash-checker
    cargo test
    cd /workspace/rust/hash-checker-gui
    cargo run --release -- --smoke-test
  SHELL
end
