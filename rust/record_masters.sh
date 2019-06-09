mkdir -p target/golden/

for seed in $(seq 1 50); do
  SEED=$seed cargo run --quiet > target/golden/${seed}.txt
done
