export default function main(arg) {
    console.log('i got called');
    console.log('got an argument with contents:');
    console.log(arg.get(0));
    console.log(arg.get(1));
}