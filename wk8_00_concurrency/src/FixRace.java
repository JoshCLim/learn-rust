class FixRace {
    static final int N_THREADS = 50;
    static final int N_INCREMENTS = 100000;

    static int my_number = 0;
    static final Object lock = new Object();

    static void thread() {
        for (int i = 0; i < N_INCREMENTS; i++) {
            synchronized (lock) {
                my_number++;
            }
        }
    }

    public static void main(String[] args) throws InterruptedException {
        Thread[] threads = new Thread[N_THREADS];

        for (int i = 0; i < N_THREADS; i++) {
            threads[i] = new Thread(DataRace::thread);
            threads[i].start();
        }

        for (int i = 0; i < N_THREADS; i++) {
            threads[i].join();
        }

        System.out.println(String.format("Final total: %d (expected %d)\n", my_number, N_THREADS * N_INCREMENTS));
    }
}
