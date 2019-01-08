use std::env;
use std::error::Error;
use std::io::Write;
use std::path::Path;

use gpgme::{edit, Context, Data, Protocol};
use tempfile::TempDir;

use crate::prelude::*;

pub(crate) const GPG_IDENTITY: &str = "vault@apognu.github.com";
const GPG_PUBLIC_KEY: &str = "-----BEGIN PGP PUBLIC KEY BLOCK-----
mI0EXC+s5gEEANbbtYbwJj+kM+fJjEZTP007guZRSOzQgESMc7f2P+zC6vWwTHdF
fczu9Oh7Q3KamZkEhP6R0TbHPY8fxKyP1kem+c4t2zDZyA2giCGmnhIj4szlJrK5
AWkRloNbYjjDjBucrK/DGRgZ7twon6zJy2Wdfzmo4IOQoO7NqSAHIVklABEBAAG0
JVZhdWx0IFRlc3RzIDx2YXVsdEBhcG9nbnUuZ2l0aHViLmNvbT6IzgQTAQgAOBYh
BK/WdXChpxNPkdkOpzgcifui4NkgBQJcL6zmAhsDBQsJCAcDBRUKCQgLBRYDAgEA
Ah4BAheAAAoJEDgcifui4NkgoYMD/iqUARnQIF0c8dSdAxGU9D7r4FD1Jhb+/wAj
soejBzXIBzrwGL4L2/BPSPmZN+n9dD3/XwxTgNw4twTbBRf4ay/6tHUJGnsI8ToA
GNZ78L/+Q+IeYjAFsS0Twk8yxblLbTLQMA52GGjUeB4doxgWiNTXCGh6lByZxVcu
gHbO9tyfuI0EXC+s5gEEALH8soBR/aPCNFfoZP6ggHTr0oAJ1121xeF4EnrooWNN
XoVyYqipmYULYgye4Xy1h50NsR1nz42OtMD4l8416YnacePgi9a5BWwIy6ZexsIl
tr4i+JNDzQxJlN/p80HvjdaS4NnwuVkOqC1sVh75ybrkPZv+5uEMJRq2BjpaD1tR
ABEBAAGItgQYAQgAIBYhBK/WdXChpxNPkdkOpzgcifui4NkgBQJcL6zmAhsMAAoJ
EDgcifui4NkgsAED/1Rudmc14lHfslXLocvZvQUrIf7AkjCEJHEae2D1zvzx80U8
Str1SX5I1Gxdj8O34V+wG9gexEEyZjFz5rdMUQpCXBRThPq6i+zwGnPuooxaRGVd
QeI+6Dbu/zTJ3+kH6qII4hlu0dz058YqX2eEsVY2cBNcYAfA/6+r8o2fzAjy
=b0sP
-----END PGP PUBLIC KEY BLOCK-----";
const GPG_SECRET_KEY: &str = "-----BEGIN PGP PRIVATE KEY BLOCK-----
lQHYBFwvrOYBBADW27WG8CY/pDPnyYxGUz9NO4LmUUjs0IBEjHO39j/swur1sEx3
RX3M7vToe0NympmZBIT+kdE2xz2PH8Ssj9ZHpvnOLdsw2cgNoIghpp4SI+LM5Say
uQFpEZaDW2I4w4wbnKyvwxkYGe7cKJ+syctlnX85qOCDkKDuzakgByFZJQARAQAB
AAP8DumewesrvgtC3pzpED2zIF4RWeec/nICrRjrtGiyWCSeBtTbm2pzbrfMWZ4Q
knhAUYdBE3A1S3sgGGDil2PzKDmTredBxgSx9DgSFdVbaWzoZbK+Jlq/eqfZwHl7
TIEb3JjKyBG8CkACSxilIwTV5bVYR+l2QBcubsSkVTc+d2MCAN2NvpbERYAoGQO7
LLp0aaWkTeA0IpClGHvoe5GWOWMAPz4ZBm3Jjwy6EnKlpqJ+p9wEMiUI3DjzA6af
O7Mau8MCAPhDeRcdvHiEtDuN8hP3hm0FbwiaXf7ehgguQFYanCk4NMgxVlBhSuHU
M0F/grJZcl3ZWeOtg5d+ZhMdk0leEPcB/Alf1ipW0PmxGlfZayvtxyp1DXRDZXKG
oc9kMIZNykTZh5yWeKmHRxv7urspgyQFBh+LvB0ebDM2Vg3BrfLmDqOT1rQlVmF1
bHQgVGVzdHMgPHZhdWx0QGFwb2dudS5naXRodWIuY29tPojOBBMBCAA4FiEEr9Z1
cKGnE0+R2Q6nOByJ+6Lg2SAFAlwvrOYCGwMFCwkIBwMFFQoJCAsFFgMCAQACHgEC
F4AACgkQOByJ+6Lg2SChgwP+KpQBGdAgXRzx1J0DEZT0PuvgUPUmFv7/ACOyh6MH
NcgHOvAYvgvb8E9I+Zk36f10Pf9fDFOA3Di3BNsFF/hrL/q0dQkaewjxOgAY1nvw
v/5D4h5iMAWxLRPCTzLFuUttMtAwDnYYaNR4Hh2jGBaI1NcIaHqUHJnFVy6Ads72
3J+dAdgEXC+s5gEEALH8soBR/aPCNFfoZP6ggHTr0oAJ1121xeF4EnrooWNNXoVy
YqipmYULYgye4Xy1h50NsR1nz42OtMD4l8416YnacePgi9a5BWwIy6ZexsIltr4i
+JNDzQxJlN/p80HvjdaS4NnwuVkOqC1sVh75ybrkPZv+5uEMJRq2BjpaD1tRABEB
AAEAA/4ypnC9p5eAdJGkupOTCmXD4CAlI6fQGRxYz2yi4XSb57aQTz7YNHtlqxmZ
8dTFQnt3LCBM9+/Ont+9UoEQw7LTeGKDkT9t4RBFZutx5HdJc+KzeWiqHfE/GKa1
nNSUc7l3UyB+K4mqIx5Yu0J0C920YOFP/7zde7hD1XbA8nOpSQIAy+38I5R5ApFu
9uW1zvRHpIkA64zNWpgs70RKm7ZKiCnLKPABRwPWtisHPMV9uNtlACVyBaypupQ9
ev81zei2awIA3271jOHa1kvtmPSrvPdgaHgCLVgDbMxDEFTVt4Sg+jYE0SOy+CwV
X6XBG+RtHag+k0bZfoARQrebSr/9sfIMMwIAlk9uT8+FN9Z6kOKopWzyXfFZE9i7
+R/NHS44YwYBEmhB2DhfDAcdR3vUzmifk3SK7Y9z2XblCJLmdTu9YvaPXKYtiLYE
GAEIACAWIQSv1nVwoacTT5HZDqc4HIn7ouDZIAUCXC+s5gIbDAAKCRA4HIn7ouDZ
ILABA/9UbnZnNeJR37JVy6HL2b0FKyH+wJIwhCRxGntg9c788fNFPEra9Ul+SNRs
XY/Dt+FfsBvYHsRBMmYxc+a3TFEKQlwUU4T6uovs8Bpz7qKMWkRlXUHiPug27v80
yd/pB+qiCOIZbtHc9OfGKl9nhLFWNnATXGAHwP+vq/KNn8wI8g==
=FOJS
-----END PGP PRIVATE KEY BLOCK-----";

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum EditorState {
  Start,
  Trust,
  Ultimate,
  Okay,
  Quit,
}

impl Default for EditorState {
  fn default() -> Self {
    EditorState::Start
  }
}

#[derive(Default)]
struct Editor;

impl edit::Editor for Editor {
  type State = EditorState;

  fn next_state(
    state: Result<Self::State, gpgme::Error>,
    status: edit::EditInteractionStatus,
    need_response: bool,
  ) -> Result<Self::State, gpgme::Error> {
    use self::EditorState as State;

    if !need_response {
      return state;
    }

    if status.args() == Ok(edit::PROMPT) {
      match state {
        Ok(State::Start) => Ok(State::Trust),
        Ok(State::Ultimate) => Ok(State::Quit),
        Ok(State::Okay) | Err(_) => Ok(State::Quit),
        Ok(State::Quit) => state,
        _ => Err(gpgme::Error::GENERAL),
      }
    } else if (status.args() == Ok("edit_ownertrust.value")) && (state == Ok(State::Trust)) {
      Ok(State::Ultimate)
    } else if (status.args() == Ok("edit_ownertrust.set_ultimate.okay"))
      && (state == Ok(State::Ultimate))
    {
      Ok(State::Okay)
    } else {
      Err(gpgme::Error::GENERAL)
    }
  }

  fn action<W: Write>(&self, state: Self::State, mut out: W) -> Result<(), gpgme::Error> {
    use self::EditorState as State;

    match state {
      State::Trust => out.write_all(b"trust")?,
      State::Ultimate => out.write_all(b"5")?,
      State::Okay => out.write_all(b"y")?,
      State::Quit => write!(out, "{}", edit::QUIT)?,
      _ => return Err(gpgme::Error::GENERAL),
    }

    Ok(())
  }
}

pub fn setup() -> TempDir {
  let tmp = tempfile::tempdir().expect("could not create temporary directory");

  let mut context =
    Context::from_protocol(Protocol::OpenPgp).expect("could not create GPG context");
  context.set_armor(true);

  context
    .import(Data::from_bytes(&GPG_SECRET_KEY).expect("could not read GPG key"))
    .expect("could not import GPG secret key");

  context
    .import(Data::from_bytes(&GPG_PUBLIC_KEY).expect("could not read GPG key"))
    .expect("could not import GPG secret key");

  env::set_var("VAULT_PATH", tmp.path());

  let key = context
    .get_key("AFD67570A1A7134F91D90EA7381C89FBA2E0D920")
    .expect("could not get GPG key");

  context
    .edit_key_with(&key, Editor, &mut Vec::new())
    .expect("could not set key trust level");

  tmp
}

pub fn get_test_vault<P>(path: P) -> Result<VaultHandle, Box<dyn Error>>
where
  P: AsRef<Path>,
{
  let handle = VaultHandle::create(&path, GPG_IDENTITY)?;
  handle.write()?;

  Ok(handle)
}