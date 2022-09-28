use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        Result
    },
    Pbkdf2
};

pub fn hash(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Pbkdf2.hash_password(password.as_bytes(), &salt)?.to_string())
}

pub fn verify(password: &String, hashed_password: &String) -> Result<()> {
    Pbkdf2.verify_password(password.as_bytes(), &PasswordHash::new(&hashed_password)?)?;
    Ok(())
}