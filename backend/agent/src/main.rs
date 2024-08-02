

struct Node {
    node_id : String,
    smart_meter : Option<SmartMeter>,
    generator : Option<Generator>
}

impl Node {
    fn new(smart_meter : Option<SmartMeter>, generator :Option<Generator>) -> Node {
        return Node {
            node_id: String::new(),
            smart_meter,
            generator,
        }
    }
}

struct SmartMeter {
    consumption_curve : Box<dyn Curve>
}


struct Generator {
    production_curve : Box<dyn Curve>
}

trait Curve {
   fn sample(&mut self,time:f64)->f64;
}

struct SineCurve;

impl SineCurve {
    fn new() -> SineCurve {
        return SineCurve;
    }
}

impl Curve for SineCurve{
    fn sample(&mut self,time:f64)->f64 {
        return f64::sin(time)
    }
}


struct Agent {
   nodes : Vec<Node>,
   units_bought : f64,
   units_sold : f64,
   funds : f64,
   extarnal_wealth_curve : Box<dyn Curve>
}

impl Agent {
    fn new(nodes : Vec<Node>,funds : f64,extarnal_wealth_curve :Box::<dyn Curve>) -> Agent {
        return Agent {
            nodes,
            units_bought: 0.0,
            units_sold: 0.0,
            funds,
            extarnal_wealth_curve,
        }
    }

    fn intialise(&mut self) {

    }

    fn update(&mut self) -> Result<(),()> {
        return Ok(());
    }

    fn run(&mut self) {
        self.intialise();
        loop {
        
        let result = self.update();

        match result {
            Ok(_) => {},
            Err(_) => {break},
        }

        }
    }
}


fn main() {
   let agent = Agent::new(vec![Node::new(None,None)],0.0,Box::new(SineCurve::new())); 
}
