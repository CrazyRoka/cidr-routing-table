use cidr_routing_table::{
    get_cidr_mask, HashRoutingTable, Ipv4Cidr, ListRoutingTable, RoutingTable, TrieRoutingTable,
};
use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
};
use mimalloc::MiMalloc;
use rand::prelude::*;
use std::{iter::repeat_with, net::Ipv4Addr};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn generate_cidr(bits: u32, len: u8) -> Ipv4Cidr {
    let mask = get_cidr_mask(len).expect("Len should be smaller than equal to 32");
    let new_bits = bits & mask;
    let addr = Ipv4Addr::from(new_bits);

    Ipv4Cidr::new(addr, len).expect("Input is correct")
}

fn bench_routing_table(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    // let sizes = [10, 100, 1000, 10000, 100000, 1000000];
    let sizes = [10000];
    let mut rng = rand::thread_rng();
    let mut group = c.benchmark_group("CidrManager");
    group.plot_config(plot_config);

    for size in sizes {
        let cidrs = repeat_with(|| generate_cidr(rng.gen(), rng.gen_range(0..=32)));
        let mut trie_routing_table = TrieRoutingTable::new();
        let mut hash_routing_table = HashRoutingTable::new();
        let mut list_routing_table = ListRoutingTable::new();

        for cidr in cidrs {
            if hash_routing_table.size() >= size {
                break;
            }

            trie_routing_table.add_cidr(cidr);
            hash_routing_table.add_cidr(cidr);
            list_routing_table.add_cidr(cidr);
        }

        println!("Table size: {}", hash_routing_table.size());

        group.sample_size(1000);
        group.bench_function(BenchmarkId::new("HashCidrManager", size), |b| {
            let mut addresses = repeat_with(|| Ipv4Addr::from(rng.gen::<u32>()));

            b.iter_batched(
                || addresses.next().unwrap(),
                |addr| {
                    hash_routing_table.find_exact_cidr(addr);
                },
                criterion::BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::new("TrieCidrManager", size), |b| {
            let mut addresses = repeat_with(|| Ipv4Addr::from(rng.gen::<u32>()));

            b.iter_batched(
                || addresses.next().unwrap(),
                |addr| {
                    trie_routing_table.find_exact_cidr(addr);
                },
                criterion::BatchSize::SmallInput,
            );
        });
        // group.bench_function(BenchmarkId::new("ListCidrManager", size), |b| {
        //     let mut addresses = repeat_with(|| Ipv4Addr::from(rng.gen::<u32>()));

        //     b.iter_batched(
        //         || addresses.next().unwrap(),
        //         |addr| {
        //             list_routing_table.find_exact_cidr(addr);
        //         },
        //         criterion::BatchSize::SmallInput,
        //     );
        // });

        println!("Finished tests with size {}", size);
    }

    group.finish();
}

criterion_group!(benches, bench_routing_table);
criterion_main!(benches);
