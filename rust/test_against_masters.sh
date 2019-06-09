set -e

mkdir -p target/test/

for seed in $(seq 1 50); do
  SEED=$seed cargo run --quiet > target/test/${seed}.txt

  cmp --silent target/test/${seed}.txt target/golden/${seed}.txt || (echo "seed ${seed} is different" && exit 1)
done

echo "All good!"
