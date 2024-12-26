fn main() {
    let mut info : Vec<String> = Vec::new(); 
    let a = vec!["Intern", "1-2", "APS 1-2"];
    let b = vec!["Paralegal","1-2", "APS 1-2"];
    let c = vec!["Placement","1-2", "APS 1-2"];
    let d = vec!["Administrator","3-5", "APS 3-5"];
    let e = vec!["Research Assistant","3-5", "APS 3-5"];
    let f = vec!["Junior Associate","3-5","APS 3-5"];
    let g = vec!["Classroom Teacher","3-5", "APS 3-5"];
    let h = vec!["Senior Administrator","5-8", "APS 5-8"];
    let _i = vec!["PhD Candidate","5-8", "APS 5-8"];
    let j = vec!["Associate","5-8", "APS 5-8"];
    let k = vec!["Senior Teacher","5-8", "APS 5-8"];
    let l = vec!["Office Manager","8-10", "EL1"];
    let m = vec!["Post-Doc Researcher","8-10", "EL1"];
    let n = vec!["Senior Associate 1-2","8-10", "EL1"];
    let o = vec!["Leading Teacher","8-10", "EL1"];
    let p = vec!["Director","10-13", "EL2"];
    let q = vec!["Senior Lecturer","10-13", "EL2"];
    let r = vec!["Senior Associate 3-4","10-13", "EL2"];
    let s = vec!["Deputy Principal","10-13", "EL2"];
    let t = vec!["CEO","13-30", "SES"];
    let u = vec!["Dean","13-30", "SES"];
    let v = vec!["Partner","13-30", "SES"];
    let w = vec!["Principal","13-30", "SES"];
    let mut index:usize = 0; 

    println!("Here are the sectors available in F.G.N:");
    println!("Office Admin, Academic, Lawyer, Teacher");
    
    let mut input1 = String::new();
    println!("Which sector do you work in?(Each word should start with a capital letter!) ");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let job_title:String = input1.trim().parse().expect("Invalid input");
    info.push(job_title.clone());
 
    let mut input2 = String::new();
    println!("How many years of experience do you have?");
    std::io::stdin().read_line(&mut input2).expect("failed to read input");
    let exp:u32 = input2.trim().parse().expect("Invalid input");
    info.push(exp.to_string());
    
    if job_title == "Intern" && exp>=1 && exp<=2 
    {
      println!("The staff memmber with {} job title and {} years experience holds position {}",a[0],a[1],a[2]);
    }
    if job_title== "Paralegal" && exp>=1 && exp<=2
        {
          println!("The staff member with {} job title and {} years experience holds position {}",b[0],b[1],b[2]);
        }
    if job_title== "Placement" && exp>=1 && exp<=2 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",c[0],c[1],c[2]);
        } 
    if job_title== "Administrator" && exp>=3 && exp<=5 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",d[0],d[1],d[2]);
        } 

   if job_title== "Research Assistant" && exp>=3 && exp<=5 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",e[0],e[1],e[2]);
        } 
   else if job_title== "Junior Associate" && exp>=3 && exp<=5 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",f[0],f[1],f[2]);
        } 
   else if job_title== "Classroom Teacher" && exp>=3 && exp<=5 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",g[0],g[1],g[2]);
        }
    else if job_title== "Senior Administrator" && exp>=5 && exp<=8 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",h[0],h[1],h[2]);
        } 
    else if job_title== "PhD Candidate" && exp>=5 && exp<=8 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",_i[0],_i[1],_i[2]);
        } 
    else if job_title== "Associate" && exp>=5 && exp<=8
        {
          println!("The staff member with {} job title and {} years experience holds position {}",j[0],j[1],j[2]);
        } 
    else if job_title== "Senior Teacher" && exp>=5 && exp<=8 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",k[0],k[1],k[2]);
        } 
    else if job_title== "Office Manager" && exp>=8 && exp<=10 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",l[0],l[1],l[2]);
        } 
    else if job_title== "Post-Doc Researcher" && exp>=8 && exp<=10 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",m[0],m[1],m[2]);
        } 
    else if job_title== "Senior Associate 1-2" && exp>=8 && exp<=10 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",n[0],n[1],n[2]);
        } 
    else if job_title== "Leading Teacher" && exp>=8 && exp<=10 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",o[0],o[1],o[2]);
        } 
    else if job_title== "Director" && exp>=10 && exp<=13 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",p[0],p[1],p[2]);
        } 
    else if job_title== "Senior Lecturer" && exp>=10 && exp<=13 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",q[0],q[1],q[2]);
        } 
    else if job_title== "Senior Associate 3-4" && exp>=10 && exp<=13 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",r[0],r[1],r[2]);
        } 
    else if job_title== "Deputy Principal" && exp>=10 && exp<=13
        {
          println!("The staff member with {} job title and {} years experience holds position {}",s[0],s[1],s[2]);
        } 
    else if job_title== "CEO" && exp>=13 && exp<=30 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",t[0],t[1],t[2]);
        } 
    else if job_title== "Dean" && exp>=13 && exp<=30 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",u[0],u[1],u[2]);
        } 
    else if job_title== "Partner" && exp>=13 && exp<=30 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",v[0],v[1],v[2]);
        }
    else if job_title== "Principal" && exp>=13 && exp<=30 
        {
          println!("The staff member with {} job title and {} years experience holds position {}",w[0],w[1],w[2]);
        }
        else {
            println!("Sorry, you do not have an APS level");
        }
}


