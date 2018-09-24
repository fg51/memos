cortex_m_rt
====


this library do the Reset process


#[link_section = ".vector_table.reset_vector"]

pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

pub unsafe extern "C" fn Reset() -> !
    * zero bss
    * init data
    * enable the FPU


