use rand::Rng;


struct Perceptron{
    weights:Vec<f32>,
    bias:f32
}

struct ClassificationMetrics{
    accuracy:f32,
    true_positives:u32,
    true_negatives:u32,
    false_positives:u32,
    false_negatives:u32,
}

impl Perceptron {
    fn new(num_inputs:usize)->Self{
        let mut rng=rand::thread_rng();
        let weights=(0..num_inputs).map(|_| rng.gen_range(-1.0..1.0)).collect();
        let bias=rng.gen_range(-1.0..1.0);
        Perceptron{weights,bias}
    }
    fn feedforward(&self, input: &[f32])->f32 {
        let mut sum= self.weights
            .iter()
            .zip(input.iter())
            .map(|(w,i)|w*i)
            .sum::<f32>();
        sum+=self.bias;
        sum
        
    }
    fn activate(&self, sum:f32)->f32{
        if sum>0.0{
            return 1.0;
        }else {
            return 0.0;
        }

    }
    fn predict(&self, inputs: &[f32])-> f32{
        self.activate(self.feedforward(inputs))
    }

    fn train(&mut self, inputs: &[f32], expected_output:f32, learning_rate:f32){
        //error calculation
        let output=self.predict(inputs);
        let output_error=expected_output-output;
        

        //updating bias
        let old_bias=self.bias;
        let new_bias=old_bias+(learning_rate*output_error);
        self.bias=new_bias;

        //updating weights
        for(i,weight) in self.weights.iter_mut().enumerate(){
            *weight+=learning_rate*output_error*inputs[i];

            
        }
    }

     // Add this method inside your 'impl Perceptron' block

/// Evaluates the perceptron's performance on a given dataset.
fn evaluate(&self, test_data: &[(Vec<f32>, f32)]) -> ClassificationMetrics {
    let mut correct_predictions = 0;
    let mut true_positives = 0;
    let mut true_negatives = 0;
    let mut false_positives = 0;
    let mut false_negatives = 0;

    for (inputs, expected) in test_data {
        let prediction = self.predict(inputs);
        let expected_val = *expected;

        // Check for correct prediction
        if (prediction - expected_val).abs() < f32::EPSILON {
            correct_predictions += 1;
        }

        // Populate confusion matrix values
        if prediction == 1.0 && expected_val == 1.0 {
            true_positives += 1;
        } else if prediction == 0.0 && expected_val == 0.0 {
            true_negatives += 1;
        } else if prediction == 1.0 && expected_val == 0.0 {
            false_positives += 1;
        } else if prediction == 0.0 && expected_val == 1.0 {
            false_negatives += 1;
        }
    }

    let total_samples = test_data.len();
    let accuracy = (correct_predictions as f32 / total_samples as f32) * 100.0;
    
    // Return the results in our new struct
    ClassificationMetrics {
        accuracy,
        true_positives,
        true_negatives,
        false_positives,
        false_negatives,
    }
}   
}

fn or_gate(){
    let training_data=vec![
        (vec![0.0,0.0],0.0),
        (vec![0.0,1.0],1.0),
        (vec![1.0,0.0],1.0),
        (vec![1.0,1.0],1.0),
    ];
    let learning_rate=0.1;
    let epochs=100;
    let mut perceptron=Perceptron::new(2);
    println!("Starting training for OR Gate...");

    for epoch in 0..epochs{
        for (inputs,expected) in &training_data {
            perceptron.train(inputs, *expected, learning_rate);
            
        }
    }
    println!("Training Finished, final weights:{:?}, final bias:{:?}",perceptron.weights,perceptron.bias);

    println!("\n Testing");
    println!("Input: [0, 0], Prediction: {}", perceptron.predict(&[0.0, 0.0]));
    println!("Input: [0, 1], Prediction: {}", perceptron.predict(&[0.0, 1.0]));
    println!("Input: [1, 0], Prediction: {}", perceptron.predict(&[1.0, 0.0]));
    println!("Input: [1, 1], Prediction: {}", perceptron.predict(&[1.0, 1.0]));
    let metrics=perceptron.evaluate(&training_data);
    println!("\n--- Results ---");
    println!("Accuracy: {:.2}%", metrics.accuracy);
    println!("\nConfusion Matrix:");
    println!("  - True Positives (TP):  {}", metrics.true_positives);
    println!("  - True Negatives (TN):  {}", metrics.true_negatives);
    println!("  - False Positives (FP): {}", metrics.false_positives);
    println!("  - False Negatives (FN): {}", metrics.false_negatives);
    println!("----------------");
}
fn and_gate(){
    let training_data=vec![
        (vec![0.0,0.0],0.0),
        (vec![0.0,1.0],0.0),
        (vec![1.0,0.0],0.0),
        (vec![1.0,1.0],1.0),
    ];
    let learning_rate=0.1;
    let epochs=100;
    let mut perceptron=Perceptron::new(2);
    println!("Starting training for AND Gate...");

    for epoch in 0..epochs{
        for (inputs,expected) in &training_data {
            perceptron.train(inputs, *expected, learning_rate);
            
        }
    }
    println!("Training Finished, final weights:{:?}, final bias:{:?}",perceptron.weights,perceptron.bias);

    println!("\n Testing");
    println!("Input: [0, 0], Prediction: {}", perceptron.predict(&[0.0, 0.0]));
    println!("Input: [0, 1], Prediction: {}", perceptron.predict(&[0.0, 1.0]));
    println!("Input: [1, 0], Prediction: {}", perceptron.predict(&[1.0, 0.0]));
    println!("Input: [1, 1], Prediction: {}", perceptron.predict(&[1.0, 1.0]));
    let metrics=perceptron.evaluate(&training_data);
    println!("\n--- Results ---");
    println!("Accuracy: {:.2}%", metrics.accuracy);
    println!("\nConfusion Matrix:");
    println!("  - True Positives (TP):  {}", metrics.true_positives);
    println!("  - True Negatives (TN):  {}", metrics.true_negatives);
    println!("  - False Positives (FP): {}", metrics.false_positives);
    println!("  - False Negatives (FN): {}", metrics.false_negatives);
    println!("----------------");
}


fn main() {
    println!("Hello Logic Gates!");
    and_gate();
    or_gate();
}
