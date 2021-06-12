#include<iostream>
#include<sstream>
using namespace std;

int main(){
    string line = "4*8+2/4-1";
        double result=0;
        double num1=0,num2=0;
        stringstream bag;
        bag<<line;
        bag>>num1;
        char ope;
        while(bag>>ope>>num2){
            switch (ope)
            {
            case '+':
                result+=num1;
                num1=num2;
                break;
            case '-':
                result+=num1;
                num2=-num2;
                num1=num2;
                break;
            case '*':
                num1*=num2;
                break;
            case '/':
                num1/=num2;
                break;
            }
        }
        result+=num1;
        printf("%.2f\n",result);
}