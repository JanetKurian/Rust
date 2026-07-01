#include <iostream>
#include <pthread.h>
#include <unistd.h>
#include <vector>
#include <cmath>
#include <fstream>

#define RUN_TIME 15

pthread_mutex_t lock;

/* ---------- ENUM ---------- */
enum operation_state {
    OFF,
    ON,
    RUNNING,
    COMPLETED
};

/* ---------- CLASS ---------- */
class PowerSupplyUnit {
public:
    int id;
    int voltageRating;
    int currentRating;
    operation_state state;

    std::vector<float> voltage_log;
    std::vector<float> current_log;

    PowerSupplyUnit(int id) {
        this->id = id;
        voltageRating = 60;
        currentRating = 30;
        state = OFF;
    }

    void configure(int voltage, int currentLimit) {
        voltageRating = voltage;
        currentRating = currentLimit;
    }

    void measure() {
        float voltage = 50 + rand() % 20;
        float current = 20 + rand() % 10;

        pthread_mutex_lock(&lock);

        voltage_log.push_back(voltage);
        current_log.push_back(current);

        pthread_mutex_unlock(&lock);
    }

    void analyzeAndStore(std::ofstream &file) {
        int size = voltage_log.size();

        float sumV = 0, sumC = 0;
        for (int i = 0; i < size; i++) {
            sumV += voltage_log[i];
            sumC += current_log[i];
        }

        float avgV = sumV / size;
        float avgC = sumC / size;

        float varV = 0, varC = 0;
        for (int i = 0; i < size; i++) {
            varV += pow(voltage_log[i] - avgV, 2);
            varC += pow(current_log[i] - avgC, 2);
        }

        float stdV = sqrt(varV / size);
        float stdC = sqrt(varC / size);

        file << "PowerSupply " << id << "\n";
        file << "Avg Voltage: " << avgV << "\n";
        file << "Std Voltage: " << stdV << "\n";
        file << "Avg Current: " << avgC << "\n";
        file << "Std Current: " << stdC << "\n\n";
    }
};

/* ---------- THREAD ARG ---------- */
struct ThreadArg {
    PowerSupplyUnit *psu;
    float interval;
};

/* ---------- THREAD FUNCTION ---------- */
void* thread_function(void* arg) {
    ThreadArg *t = (ThreadArg*)arg;

    int iterations = RUN_TIME / t->interval;

    t->psu->state = RUNNING;

    for (int i = 0; i < iterations; i++) {
        t->psu->measure();
        usleep(t->interval * 1000000);
    }

    t->psu->state = COMPLETED;

    pthread_exit(NULL);
}

/* ---------- MAIN ---------- */
int main() {

    pthread_t threads[3];
    ThreadArg args[3];

    pthread_mutex_init(&lock, NULL);

    /* Create 3 PSU objects */
    PowerSupplyUnit psu1(1);
    PowerSupplyUnit psu2(2);
    PowerSupplyUnit psu3(3);

    PowerSupplyUnit* psuArray[3] = {&psu1, &psu2, &psu3};

    float intervals[3] = {0.5, 1.0, 1.0};

    /* Create threads */
    for (int i = 0; i < 3; i++) {
        args[i].psu = psuArray[i];
        args[i].interval = intervals[i];

        pthread_create(&threads[i], NULL, thread_function, &args[i]);
    }

    /* Join threads */
    for (int i = 0; i < 3; i++) {
        pthread_join(threads[i], NULL);
    }

    /* Store results */
    std::ofstream file("analysis.txt");

    for (int i = 0; i < 3; i++) {
        psuArray[i]->analyzeAndStore(file);
    }

    file.close();

    pthread_mutex_destroy(&lock);

    std::cout << "Execution complete. Data stored in analysis.txt\n";

    return 0;
}