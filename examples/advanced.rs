use rust_native_obf::*;

fn main() {
    println!("advanced obfuscation demo\n");

    println!("=== multiple string obfuscation ===");
    let s1 = obf_str!("first secret");
    let s2 = obf_str!("second secret");
    let s3 = obf_str!("third secret");
    println!("{} | {} | {}", s1, s2, s3);

    println!("\n=== obfuscated block execution ===");
    let result = obf_block!({
        let mut sum = 0;
        for i in 0..10 {
            sum += i;
        }
        sum
    });
    println!("computed in obfuscated block: {}", result);

    println!("\n=== cascade encryption demo ===");
    let original = b"this is some sensitive data that needs protection";
    println!("original length: {}", original.len());
    
    let enc1 = cascade_encrypt(original, 3);
    let enc2 = cascade_encrypt(original, 5);
    let enc3 = cascade_encrypt(original, 10);
    
    println!("encrypted with 3 rounds: {:?}", &enc1[..10]);
    println!("encrypted with 5 rounds: {:?}", &enc2[..10]);
    println!("encrypted with 10 rounds: {:?}", &enc3[..10]);
    
    let dec = cascade_decrypt(&enc3, 10);
    println!("decrypted matches: {}", original.as_slice() == &dec[..]);

    println!("\n=== xor stream encryption ===");
    let mut data = b"hello world".to_vec();
    let key = b"secret";
    println!("original: {:?}", String::from_utf8_lossy(&data));
    
    xor_stream(&mut data, key);
    println!("encrypted: {:?}", &data);
    
    xor_stream(&mut data, key);
    println!("decrypted: {:?}", String::from_utf8_lossy(&data));

    println!("\n=== compile time constants ===");
    let c1 = obf_const!(0xdeadbeef, u32);
    let c2 = obf_const!(0xcafebabe12345678, u64);
    println!("const 1: {:#x}", c1);
    println!("const 2: {:#x}", c2);

    println!("\n=== wide string operations ===");
    let w1 = wide_str!("ASCII");
    let w2 = wide_str!("Unicode: 你好");
    let w3 = wide_str!("Emoji: 🚀🔥");
    
    println!("w1 len: {}", w1.len());
    println!("w2 len: {}", w2.len());
    println!("w3 len: {}", w3.len());
    
    let obf_w1 = obf_wide!("Obfuscated Wide");
    println!("obfuscated wide len: {}", obf_w1.len());

    println!("\n=== multiple random values ===");
    for i in 0..5 {
        let r = ct_rand!(u32, stringify!(i));
        println!("random {}: {:#x}", i, r);
    }

    println!("\n=== hash comparisons ===");
    let data1 = b"test data 1";
    let data2 = b"test data 2";
    
    let h1 = ct_xxhash!(data1);
    let h2 = ct_xxhash!(data2);
    
    println!("hash of data1: {:#x}", h1);
    println!("hash of data2: {:#x}", h2);
    println!("hashes different: {}", h1 != h2);

    println!("\n=== obfuscated value storage ===");
    let values = vec![
        ObfuscatedValue::new(111u32),
        ObfuscatedValue::new(222u32),
        ObfuscatedValue::new(333u32),
    ];
    
    for (i, val) in values.iter().enumerate() {
        println!("value {}: {}", i, val.get());
    }

    println!("\n=== complex computation in hidden call ===");
    let computed = hidden_call!({
        let mut result = 1u64;
        for i in 1..=10 {
            result *= i;
        }
        result
    });
    println!("10! = {}", computed);

    println!("\n=== nested obfuscation ===");
    let nested = hidden_call!({
        obf_block!({
            let s = obf_str!("nested secret");
            s.len() * 2
        })
    });
    println!("nested result: {}", nested);

    println!("\nadvanced demo completed!");
}

