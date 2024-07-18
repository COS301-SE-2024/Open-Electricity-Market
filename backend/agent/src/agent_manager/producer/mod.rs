
pub struct Voltage(f32);


#[derive(Debug)]
struct ProducerInit {
    tx: u32 //Since I need to still check type
}


pub trait Producer {
   fn get_voltage(&self,elasped_time :f32) -> Voltage;
   fn update(&self,elasped_time :f32) {
        // Get voltage
        let voltage = self.get_voltage(elasped_time);

        //Check Database to see if I have sold units
        let units_sold = 0.0;
        //Make Desision
        if units_sold == 0.0 {
            //Tell manager and stop
        } else {
            //Upadate units sold and tell database 
        }

        //Every 5 mintues tell grid about my voltages

   }
}
