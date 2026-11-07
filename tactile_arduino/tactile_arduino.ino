#define PIN 3

void setup() {
  Serial.begin(9600);
  pinMode(PIN, OUTPUT);
  analogWrite(PIN, 0);
}

void loop() {
  if (Serial.available()) {
    int value = Serial.parseInt();
    // Make sure it is clean for the next read
    if (Serial.available()) {
      Serial.read(); 
    }
    if (value > 0) {
      analogWrite(PIN, 255);
      delay(value);
      analogWrite(PIN, 0);
    }
  }
}
