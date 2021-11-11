/*
&data==data_ref
说明进行了浅拷贝
&&data!=&data_ref!=&data_ref_insum
&data[0]==&data_ref[0]==&data_ref_insum[0]
 */

fn main() {
    let data = vec![1, 2, 3, 4];
    let data_ref = &data;

    println!("addr of &data: {:p}", &data);
    println!("addr of data_ref: {:p}", data_ref);
    println!("addr of &&data {:p}", &&data);
    println!("addr of &data_ref {:p}", &data_ref);

    sum(data_ref);

    println!("addr of item[0] in data: {:p}", &data[0]);
    println!("addr of item[0] in data_ref: {:p}", &data_ref[0]);
}

fn sum(data_ref_insum: &[u32]) -> u32 {
    println!("addr of &data_ref_insum: {:p}", &data_ref_insum);
    println!("addr of item[0] in data_ref_insum: {:p}", &data_ref_insum[0]);
    data_ref_insum.iter().sum()
}
