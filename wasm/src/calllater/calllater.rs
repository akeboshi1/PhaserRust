
impl FnCallLater{
    pub fn addCallLater<T,U,V>(&self,f:T,u:U,v:v)
    where T:,
          U:,
          V:
    {
       &self.calllaterFnVec.push(f);
       &self.calllaterContext.push(u);
       &self.calllaterParams.push(v);
    }

    pub fn calllater(&self){
        let calllaterFnVec = &self.calllaterFnVec;
        let calllaterContext = &self.calllaterFnVec;
        let calllaterParams = &self.calllaterParams;
        for i in calllaterFnVec {
           // todo deal fncalllater
        }
    }
}

pub struct FnCallLater{
    calllaterFnVec:Vec<T>,
    calllaterContext:Vec<T>,
    calllaterParams:Vec<T>
}