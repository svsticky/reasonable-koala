use base64::Engine;
use sha2::Digest;

const BCRYPT_COST: u32 = 10;
pub const SALT_LENGTH: usize = 16;

/// Generate a hash for the provided input.
/// The returned String is a hash of the input and it's salt
///
/// # Panics
///
/// - When the provided `salt` is not exactly 16 characters long
///
/// # Errors
///
/// If hashing fails
///
/// # Panics
///
/// If the provided salt is not *exactly* 16 bytes long
pub fn hash(input: &str, salt: &str, pepper: &str) -> Result<String, bcrypt::BcryptError> {
    let mut hasher = sha2::Sha512_256::new();

    hasher.update(input);
    hasher.update(pepper);

    let salt_bytes = salt.as_bytes();
    if salt_bytes.len() != 16 {
        panic!("Salt is not 16 bytes long")
    }
    let mut salt_bytes_arr = [0_u8; 16];
    salt_bytes_arr.copy_from_slice(salt_bytes);

    let engine = base64::prelude::BASE64_STANDARD;
    let hash = engine.encode(hasher.finalize());
    let bcrypt = bcrypt::hash_with_salt(hash, BCRYPT_COST, salt_bytes_arr)?.format_for_version(bcrypt::Version::TwoB);

    Ok(bcrypt)
}

/// Verify an input is the same as the stored hash. The same `pepper` must be used
///
/// # Errors
///
/// If verifying fails
pub fn verify(stored_hash: &str, input: &str, pepper: &str) -> Result<bool, bcrypt::BcryptError> {
    let mut hasher = sha2::Sha512_256::new();

    hasher.update(input);
    hasher.update(pepper);

    let engine = base64::prelude::BASE64_STANDARD;
    let hash = engine.encode(hasher.finalize());

    let correct = bcrypt::verify(hash, stored_hash)?;

    Ok(correct)
}

#[cfg(test)]
mod test {
    use super::{hash, verify};

    #[test]
    fn long_password() {
        let password = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz123456789!@#$%^&*()_+=-.,/<>?ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz123456789!@#$%^&*()_+=-.,/<>?ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz123456789!@#$%^&*()_+=-.,/<>?";

        assert!(hash(password, "0123456789ABCDEF", "Baz").is_ok());
    }

    #[test]
    fn utf8() {
        let password = r#"1ǯ;ؔ洲񏒡lٯ>睵帠t0򴢷򞈚𠁺ꪲ2ɧޞ顬أz형Ξ򢏟򇶔Cǣ>𾊅ܱ
            ƒd󚯋뭔̈́򾠄Ú򈭦廃i墼񺭺뼣犔G:絫ֵ藦၏O񃾴0̆Ɲ~򳯠μ񏅲¡
            򀛧񼸎҅󇛚▍ӥ஌}謹쪊񆷚荩簝ڋ򄔵~Ê΂湰ǈ򏯃󔬝ȑ)Ͻe0Bݻ󺦠ꯧG
            [񯹣٪즯P񄕤𗚕Ę;狥#񆝤뿇+kلެ󿧾ðzj2哳nܝ얂񐰅ևܐߍԞ菟
            ߺáJ՞ϴQѲ䆑䚁夸ʘQ꒾㖊򵤅𫨻騌Ӽ񱢻տ�Թ!֦j"󏮢봸򰍲Ґ􁙟"#;

        assert!(hash(password, "0123456789ABCDEF", "Baz").is_ok());

        let password = r#"꩘>õ阭ܪ򤙩ԁʢΝO
            񗎕ē𼖝򜬳󀤬񎘰{򗲒؃@
            ?1朼갗ʹ򻖜㈌􌟴暻F
            互O嗎񧠙挐E򨃬<񨋥г
            󧬧ꎃٛՔ6䲋ړP䃩
            ńʏ񖸋Ɗ񛏨󎄅<�f
            K󧾴劉ڏ󲟶􍐭􉚐񭶙񾠱-
            ť֚ۧӟ4񚚐�򐺔򞧎Ӿ
            󚁝n撁鞌BΒh񸝕ｒݢ
            򘴱ⰲ򨅂󮸠񧎭tᖥ󎸟뛜"#;

        assert!(hash(password, "0123456789ABCDEF", "Baz").is_ok());
    }

    #[test]
    #[should_panic]
    fn incorrect_salt_len() {
        let _ = hash("Foo", "Bar", "Baz").unwrap();
    }

    #[test]
    fn correct() {
        let salt = "0123456789ABCDEF";
        let password = "XYZ";
        let pepper = "123";
        let hash = hash(password, salt, pepper).unwrap();

        assert!(verify(&hash, &password, pepper).unwrap());
    }

    #[test]
    fn incorrect() {
        let salt = "0123456789ABCDEF";
        let password = "XYZ";
        let password2 = "ABC";
        let pepper = "123";
        let hash = hash(password, salt, pepper).unwrap();

        assert!(!verify(&hash, &password2, pepper).unwrap());
    }
}