use proconio::input;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    
    /* 解説: https://atcoder.jp/contests/abc318/editorial/7068
        i<j<kの、任意のjの(i,j,k)の組み合わせの数は、
        Ai==Akなので、jを基準に左右の数字(1~N)の個数を数えて、その個数の掛け算で求まる
        Ai!=Aj,Ak!=Ajなので、最後にAjの値の組み合わせ数を引く

        例えば、9 7 2 7 3 2 2 6 7の場合で、j=4(Aj=7)の時、
            (Ai,Aj,Ak)=(2,7,2), (i,j,k)=(3,4,6),(3,4,7)の2個
        これは、j=4を基準にした際、左右の個数は
            9=左1右0, 7=左1右1, 2=左1右2, 3,6=左0右1
        i<j<k,Ai==Akの組み合わせの数は、
            (9,Aj,-)=0個, (7,Aj,7)=1個, (2,Aj,[2,2])=2個, (-,Aj,[3,6])=0個
        の総和で3
        ただAi!=Aj,Ak!=Ajなので、Aj=7のためAi=Ak=7の組み合わせ数1個を除外して、2個になる
     */
    // 
    let mut ans: i32 = 0;
    // Ai=1~N。真ん中のjを基準に登場するAiの個数をlef, rigにカウントする
    let mut lef = vec![0; n+1];
    let mut rig = vec![0; n+1];
    // j=1として右側要素の個数をrigにカウント
    for &x in &a {
        rig[x] += 1;
    }

    let mut sum = 0;
    // jを2からN-1まで動かす (1 <= i < j < k <= N)
    for j in 1..n-1 {
        let x = a[j - 1];
        lef[x] += 1;   // 左側要素の個数を1増やし、
        rig[x] -= 1;   // 右側要素の個数を1減らす

        // 計算数をO(N)にするため、lefとrigの変化部分のみsumを更新
        sum += lef[x] * rig[x];
        sum -= lef[a[j]] * rig[a[j]];

        // 答えを更新
        ans += sum;
    }

    println!("{}", ans);
}