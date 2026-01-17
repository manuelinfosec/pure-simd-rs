
fn foo() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {   
        if is_x86_feature_detected!("avx2") {
            return unsafe {
                foo_avx2();
            }
        }
    }
    // fallback implementation without using the AVX2

}

fn my_function() {
    // use runtime detection
    #[cfg(feature = "std")]
    {
        #[cfg(target_arch = "x86_64")]
        if is_x86_feature_detected!("avx512f") {
            return my_function_avx512();
        }

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        if is_x86_feature_detected!("avx2") {
            return my_function_avx2();
        }
    }

    // use compile-time detection
    #[cfg(not(feature = "std"))]
    {
        #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
        return my_function_avx512();
    
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
        return my_function_avx2();
    }

    // scalar fallback fo when SIMD acceleration is not available
    return my_function_generic();
}


fn main() {
    println!("Hello, world!");
}