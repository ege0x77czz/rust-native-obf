use rust_native_obf::*;

fn main() {
    println!("=== string obfuscation ===");
    let secret = obf_str!("super secret password");
    println!("obfuscated string: {}", secret);

    println!("\n=== compile time hashing ===");
    let hash1 = ct_xxhash!(b"test data");
    let hash2 = ct_sdbm!("test string");
    println!("xxhash: {:#x}", hash1);
    println!("sdbm hash: {:#x}", hash2);

    println!("\n=== compile time random ===");
    let r1 = ct_rand!(u32);
    let r2 = ct_rand!(u64, "seed1");
    println!("random u32: {}", r1);
    println!("random u64: {}", r2);

    println!("\n=== value encoding ===");
    let encoded_val = obf_const!(42069, u32);
    println!("decoded constant: {}", encoded_val);
    
    let rt_enc = encoding::runtime_encode(12345);
    let rt_dec = encoding::runtime_decode(rt_enc);
    println!("runtime encode/decode: {} -> {} -> {}", 12345, rt_enc, rt_dec);

    println!("\n=== hidden execution ===");
    let result = hidden_call!({
        compute_sensitive(100)
    });
    println!("hidden call result: {}", result);

    println!("\n=== obfuscated if ===");
    let x = 10;
    obf_if!(x > 5, {
        println!("x is greater than 5");
    }, {
        println!("x is less or equal to 5");
    });

    println!("\n=== cascade encryption ===");
    let data = b"sensitive information here";
    let encrypted = cascade_encrypt(data, 5);
    let decrypted = cascade_decrypt(&encrypted, 5);
    println!("cascade works: {}", data.as_slice() == &decrypted[..]);

    println!("\n=== obfuscated bytes ===");
    let bytes = obf_bytes!(b"secret bytes");
    println!("decrypted bytes: {:?}", &bytes[..]);

    println!("\n=== wide string ===");
    let wide = wide_str!("Hello 🌍");
    println!("wide string length: {}", wide.len());
    
    let obf_w = obf_wide!("Wide String");
    println!("obfuscated wide length: {}", obf_w.len());

    println!("\n=== static reference obfuscation ===");
    static TEST_DATA: [u8; 4] = [1, 2, 3, 4];
    let obf_ref = obf_static_ref!(&TEST_DATA);
    println!("obfuscated reference: {:?}", obf_ref);

    println!("\n=== obfuscated value ===");
    let obf_val = ObfuscatedValue::new(123456789u64);
    println!("stored and retrieved: {}", obf_val.get());

    println!("\n=== noise functions ===");
    let noise = noise_loop(50);
    let fake = fake_compute(42);
    println!("noise: {}, fake compute: {}", noise, fake);

    println!("\n=== anti debug check ===");
    if anti_debug() {
        println!("no debugger detected");
    } else {
        println!("debugger present!");
    }

    println!("\nall tests completed successfully!");
}

fn compute_sensitive(n: u64) -> u64 {
    fake_compute(n) % 1000
}
