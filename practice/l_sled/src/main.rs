
use {
    byteorder::{BigEndian, LittleEndian},
    zerocopy::{
        byteorder::U64, AsBytes, FromBytes, LayoutVerified, Unaligned,
    },
};

//  键结构体
//  zerocopy::byteorder::U64保证了数据对齐问题
#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
struct Key {
    a: U64<BigEndian>,
    b: U64<BigEndian>,
}

// 值结构体
#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
struct Value {
    count: U64<LittleEndian>,
    whatever: [u8; 16],
}

let key = Key { a: U64::new(21), b: U64::new(890) };

// 取得键所对应的值，并对其施加给定函数灿做
db.update_and_fetch(key.as_bytes(), |value_opt| {
    if let Some(existing) = value_opt {
        let mut backing_bytes = sled::IVec::from(existing);

        // 验证数据对齐（这里其实不是必须的，因为我们使用了U64）
        let layout: LayoutVerified<&mut [u8], Value> =
            LayoutVerified::new_unaligned(&mut *backing_bytes)
                .expect("bytes do not fit schema");

        // 得到底层数据的可变引用
        let value: &mut Value = layout.into_mut();

        let new_count = value.count.get() + 1;

        println!("incrementing count to {}", new_count);

        value.count.set(new_count);

        Some(backing_bytes)
    } else {
        println!("setting count to 0");

        //  初始化一个Value
        Some(sled::IVec::from(
            Value { count: U64::new(0), whatever: [0; 16] }.as_bytes(),
        ))
    }
})?;

fn main() {
	let tree = sled::open("welcome").expect("open");
	tree.insert("KEY1","VAL1");
	tree.flush();
  println!("Hello, world!");
}
