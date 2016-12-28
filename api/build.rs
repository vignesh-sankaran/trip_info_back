extern crate serde_codegen;
extern crate runas;

use std::env;
use std::path::Path;
use std::fs::*;
use std::process;

fn main()
{
    #[cfg(any(feature = "dev-local-nightly", feature = "dev-local-stable"))]
    setup_local_ssl_macos();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let src = Path::new("src/serde_types.in.rs");
    let dst = Path::new(&out_dir).join("serde_types.rs");
    
    serde_codegen::expand(&src, &dst).unwrap();
}

// Optimise this first if the build times start getting too long
fn setup_local_ssl_macos()
{
    process::Command::new("security")
                    .arg("unlock-keychain")
                    .arg("-u")
                    .arg("/Library/Keychains/System.keychain")
                    .output()
                    .expect("Failed to unlock login keychain");

    // Delete all certs inside ssl directory if it exists
    if Path::new("./ssl").exists()
    {
        runas::Command::new("security")
                     .arg("delete-certificate")
                     .arg("-c")
                     .arg("localhost")
                     .status()
                     .expect("Unable to delete certificate");
        
        remove_dir_all(Path::new("./ssl")).expect("Unable to delete ssl directory");

    }

    create_dir("./ssl/").expect("Unable to create new ssl directory");

    // Create new ssl key and certificate
    process::Command::new("openssl")
                    .current_dir("./ssl/")
                    .arg("req")
                    .arg("-x509")
                    .arg("-newkey")
                    .arg("rsa:4096")
                    .arg("-keyout")
                    .arg("key.pem")
                    .arg("-out")
                    .arg("cert.pem")
                    .arg("-days")
                    .arg("60")
                    .arg("-nodes")
                    .arg("-subj")
                    .arg("/C=AU/ST=Victoria/L=Melbourne/O=Ferndrop Pty Ltd/OU=org/CN=localhost")
                    .output()
                    .expect("Failed to create ssl key and certificate");
    
    // Decrypt the ssl key
    process::Command::new("openssl")
                    .arg("rsa")
                    .arg("-in")
                    .arg("./ssl/key.pem")
                    .arg("-out")
                    .arg("./ssl/dec.pem")
                    .output()
                    .expect("Failed to decrypt the private key");
                
    // Add the certificate into the System keychain
    runas::Command::new("security")
                    .arg("add-trusted-cert")
                    .arg("-d")
                    .arg("-r")
                    .arg("trustRoot")
                    .arg("-k")
                    .arg("/Library/Keychains/System.keychain")
                    .arg("./ssl/cert.pem")
                    .status()
                    .expect("Failed to add cert to keychain");
}