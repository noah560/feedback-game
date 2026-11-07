#define PIN 3

void setup() {
  Serial.begin(9600);
  pinMode(PIN, OUTPUT);
  analogWrite(PIN, 0);
}

void loop() {
  if (Serial.available()) {
    Serial.read();
    analogWrite(PIN, 255);
    delay(200);
    analogWrite(PIN, 0);
  }
}
