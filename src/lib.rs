#[macro_use]
extern crate ocaml;
use ocaml::ToValue;

use std::mem;

macro_rules! load_vec {
    ($v:ident, $vec:ident, $block:block) => {
        let ptr = $v.field(0).mut_ptr_val();
        let len = $v.field(1).usize_val();
        let cap = $v.field(2).usize_val();

        let mut $vec: Vec<ocaml::Value> = Vec::from_raw_parts(ptr, len, cap);

        $block

        mem::forget($vec);
    }
}

macro_rules! modify_vec {
    ($v:ident, $vec:ident, $dest:ident, $block:block) => {
        load_vec!($v, $vec, {
            $block
            let new_ptr = $vec.as_mut_ptr();
            let new_len = $vec.len();
            let new_cap = $vec.capacity();
            $dest = tuple!(ocaml::Value::ptr(new_ptr), ocaml::Value::usize(new_len), ocaml::Value::usize(new_cap));
        });
    }
}

caml!(vec_create, |n|, <dest>, {
    let mut vec: Vec<ocaml::Value> = Vec::with_capacity(n.usize_val());
    let ptr = vec.as_mut_ptr();
    let len = vec.len();
    mem::forget(vec);
    dest = tuple!(ocaml::Value::ptr(ptr), len, n);
} -> dest);

caml!(vec_free, |handle|, {
    let ptr = handle.field(0).mut_ptr_val();
    let len = handle.field(1).usize_val();
    let cap = handle.field(2).usize_val();

    let vec: Vec<ocaml::Value> = Vec::from_raw_parts(ptr, len, cap);
    mem::drop(vec)
});

caml!(vec_length, |handle|, <dest>, {
    let len = handle.field(1).usize_val();
    dest = ocaml::Value::usize(len);
} -> dest);

caml!(vec_push, |handle, x|, <dest>, {
    modify_vec!(handle, vec, dest, {
        vec.push(x);
    });
} -> dest);

caml!(vec_pop, |handle|, <dest, val, tmp>, {
    modify_vec!(handle, vec, tmp, {
        val = match vec.pop() {
            Some(x) => {
                ocaml::Value::some(x)
            },
            None => ocaml::Value::none()
        };
    });
    dest = tuple!(val, ocaml::Value::from(tmp));
} -> dest);

caml!(vec_clear, |handle|, <dest>, {
    modify_vec!(handle, vec, dest, {
        vec.clear()
    });
} -> dest);

caml!(vec_index, |handle, index|, <dest>, {
    load_vec!(handle, vec, {
        if vec.len() <= index.usize_val() {
            dest = ocaml::Value::none();
        } else {
            dest = ocaml::Value::some(vec[index.usize_val()].clone())
        }
    });
} -> dest);

caml!(vec_set_index, |handle, index, x|, {
    load_vec!(handle, vec, {
        if vec.len() <= index.usize_val() {
            return
        }

        vec[index.usize_val()] = x;
    });
});