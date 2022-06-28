class Example {
public:
  int val;

  int square_val(int n);
};

int Example::square_val(int n) {
  int collect = val;

  for (int i = 0; i < n; ++i) {
    collect *= val;
  }

  return collect;
}