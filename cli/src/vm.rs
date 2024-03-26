use crate::bindings;
use crate::welcome;
use libc::c_char;
use std::ffi::CString;

pub fn run(cpus: u32, mem: u32, port: u16) {
    unsafe {
        let ctx = bindings::krun_create_ctx() as u32;

        let ret = bindings::krun_set_vm_config(ctx, cpus as u8, mem);
        if ret < 0 {
            println!("ret: {}", ret);
            println!("Error setting VM config");
            std::process::exit(-1);
        }

        let rootfs = CString::new("/Users/erikkaum/Documents/beanbox-vm/rootfs").unwrap();

        let ret = bindings::krun_set_root(ctx, rootfs.as_ptr());
        if ret < 0 {
            println!("Error setting VM rootfs");
            std::process::exit(-1);
        }

        let port_map = CString::new(format!("{}:{}", port, port)).unwrap();
        let ps: Vec<*const c_char> = vec![port_map.as_ptr(), std::ptr::null()];

        let ret = bindings::krun_set_port_map(ctx, ps.as_ptr());
        if ret < 0 {
            println!("Error setting VM port map");
            std::process::exit(-1);
        }

        let hostname = CString::new(format!("HOSTNAME={}", "beanbox")).unwrap();
        let home = CString::new("HOME=/root").unwrap();
        let env: [*const c_char; 3] = [hostname.as_ptr(), home.as_ptr(), std::ptr::null()];

        let cmd = CString::new(format!("/bin/gotty -port {} -w /bin/bash", port)).unwrap();
        let ret = bindings::krun_set_exec(ctx, cmd.as_ptr(), std::ptr::null(), env.as_ptr());
        if ret < 0 {
            println!("Error setting VM config");
            std::process::exit(-1);
        }

        println!("{}", welcome::WELCOME);
        let ret = bindings::krun_start_enter(ctx);
        if ret < 0 {
            println!("ret: {}", ret);
            println!("Error starting VM");
            std::process::exit(-1);
        }
    }
}
