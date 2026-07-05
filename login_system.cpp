#include <iostream>
#include <fstream>
#include <string.h>
using namespace std;
void login();
void registration();
void forgot();
int main()
{
	///std::cout<<"Hello World";
	int ch;
	cout<<"press 1: LOGIN"<<endl;
	cout<<"press 2: REGISTER"<<endl;
	cout<<"press 3: if you forgot PASSWORD"<<endl;
	cout<<"press 4: EXIT"<<endl;
	cin>>ch;
	switch(ch) {
	case 1:
		login();
		break;
	case 2:
		registration();
		break;
	case 3:
		forgot();
		break;
	case 4:
		cout<<"Thank you";
		break;
	default:
	//	system("cls");
		cout<<"please select from the option"<<endl;
		main();
	}

	return 0;
}


void login() {
	int count;
	string userID, password, id, pass;
//	system("cls");
	cout<<"USERNAME"<<endl;
	std::cin >> userID;
	cout<<"PASSWORD"<<endl;
	cin>>password;
	ifstream input("records.txt");
	while(input>>id>>pass) {
		if(id==userID&&pass==password) {
			count=1;
			system("cls");
		}
		input.close();
		if(count==1) {
			cout<<userID<<"login successful"<<endl;
			main();
		} else {
			cout<<"login error: please check USERNAME and PASSWORD";
			main();
		}
	}
}

void registration() {
	string ruserID,rpassword,rid,rpass;
//	system("cls");
	cout<<"USERNAME"<<endl;
	std::cin >> ruserID;
	cout<<"PASSWORD"<<endl;
	cin>>rpassword;
	ofstream f1("records.txt",ios::app);
	f1<<ruserID<<"  "<<rpassword<<endl;
//	system("cls");
		cout<<ruserID<<"registration successful"<<endl;
	main();
}

void forgot(){
    int option;
 //   	system("cls");
    	cout<<"forgot password? no worries\n";
    	cout<<"Press 1 to serach id by USERNAME"<<endl;
    	cout<<"Press 2 to return to main menu"<<endl;
    	cout<<"enter choice\n";
    	cin>>option;
    	switch(option){
    	    case 1: {
    	            	    	int count;
	string suserID, sId, spass;
//	system("cls");
	cout<<"USERNAME"<<endl;
	std::cin >> suserID;
	ifstream f2("records.txt");
	while(f2>>sId>>spass) {
		if(sId==suserID) {
			count=1;
//			system("cls");
		}
	}
		f2.close();
		if(count==1) {
			cout<<suserID<<"Your account found!"<<endl;
			cout<<"Your PASSWORD"<<spass<<endl;
			main();
		} else {
			cout<<"account not found";
		}
	
	break;
    	    }
 
	case 2: {
	    main();
	}
	default:
    	cout<<"wrong choice"<<endl;
    	}	    
    	
}