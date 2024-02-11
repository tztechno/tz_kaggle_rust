use polar::prelude::*;

fn main() {
    // 1から100までの素数を計算するタスクを作成
    let task = spawn(async {
        let mut primes = vec![2];
        for i in 3..100 {
            let mut is_prime = true;
            for prime in &primes {
                if i % prime == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                primes.push(i);
            }
        }
        primes
    });

    // タスクの結果を取得
    let primes = task.await;

    // 結果を出力
    for prime in primes {
        println!("{}", prime);
    }
}
